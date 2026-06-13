use std::{sync::Arc, thread, time::{self, Duration, SystemTime, UNIX_EPOCH}};
use all_smi::{device::GpuInfo, ui::filter_dsl::DeviceRowView};
use floem::{Application, prelude::*, reactive::{SyncRwSignal, create_effect}, taffy::*, window::*};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use crate::{gpus::GpusService, system_info::SystemInfo};
mod system_info;
mod gpus;

fn main(){
    
    let runtime = Runtime::new().expect("Could not start tokio runtime");
    let title=env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    
    let config = WindowConfig::default().title(format!("{} V{}",title.to_lowercase(),version)).theme_override(Theme::Dark);
    runtime.block_on(async{ tokio::task::block_in_place(||Application::new().window(container_view, Some(config)).run()) })
}

fn info_card(titulo: String, descripcion: String) -> impl IntoView {
    v_stack((
        label(move || titulo.clone()),
        label(move || descripcion.clone()),
    ))
    .style(|s| {
        s.padding(10)
            .gap(4)
            .min_width(120)
            .border(1)
            .border_radius(8)
            // .background(Color::from_rgba8(35, 35, 35, 35))
    })
}

fn header() -> impl IntoView {
    let system_servie = SystemInfo::new();

    v_stack((
        label(|| "System".to_string()).style(|s| {
            s.font_size(18)
                .font_bold()
                .padding(10)
        }),

        h_stack((
            info_card("O.S".into(), system_servie.os_name.to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("Kernel".into(), system_servie.kernel_version.to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("CPU".into(), system_servie.cpu.to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("Temp CPU".into(), system_servie.temp_cpu.unwrap_or_default().to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("Ram".into(), system_servie.total_ram.to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("Storage".into(), system_servie.total_storage.to_string()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
            info_card("Monitor".into(), "Active".into()).style(|s|{s.background(Color::from_rgba8(25, 25, 25, 255))}),
        ))
        .style(|s| {
            s.gap(10)
                .flex_wrap(FlexWrap::Wrap)
        }),
    ))
    .style(|s| {
        s.width_full()
            .padding(10)
    })
}

#[derive(Clone)]
pub struct MonitoringState {
    pub gpu_name:SyncRwSignal<String>,
    pub driver_version:SyncRwSignal<String>,
    pub pci_buss:SyncRwSignal<String>,
    pub gpu_ram:SyncRwSignal<f32>,
    pub gpu_ram_usage:SyncRwSignal<f32>,
    pub gpu_temp: SyncRwSignal<u32>,
    pub others_graphics: SyncRwSignal<Vec<GpuInfo>>,
}
fn gpu_card(state: MonitoringState) -> impl IntoView {
    let temp_signal = state.gpu_temp.clone();
    
    v_stack((
        // Encabezado con ícono
        h_stack((
            label(|| "🖥️ ".to_string()),
            label(|| "Active Graphics".to_string())
                .style(|s| {
                    s.font_size(20)
                        .font_bold()
                        .color(Color::from_rgba8(255, 255, 255, 255))
                }),
        ))
        .style(|s| s.gap(8).align_items(AlignItems::Center)),
        
        // Contenedor tipo tarjeta para la información
        v_stack((
            // Nombre y PCI
            h_stack((
                label(|| "Name: ".to_string()).style(|s| s.color(Color::from_rgba8(150, 150, 150, 255))),
                label(move || state.gpu_name.get().clone())
                    .style(|s| s.color(Color::from_rgba8(255, 255, 255, 255)).font_bold()),
            ))
            .style(|s| s.gap(5).padding_bottom(5)),
            
            // RAM - Barra visual o indicador
            h_stack((
                label(|| "RAM: ".to_string()).style(|s| s.color(Color::from_rgba8(150, 150, 150, 255))),
                label(move || format!("{}GB / {}GB", 
                    state.gpu_ram_usage.get().clone(),
                    state.gpu_ram.get().clone()
                ))
                .style(move |s| {
                    let usage = state.gpu_ram_usage.get().clone();
                    let total = state.gpu_ram.get().clone();
                    let percent = (usage / total) * 100.0;
                    
                    let color = if percent > 80.0 {
                        Color::from_rgba8(255, 80, 80, 255) // Rojo
                    } else if percent > 50.0 {
                        Color::from_rgba8(255, 200, 50, 255) // Amarillo
                    } else {
                        Color::from_rgba8(100, 200, 100, 255) // Verde
                    };
                    s.color(color).font_bold()
                }),
            ))
            .style(|s| s.gap(5).padding_bottom(5)),
            
            // Temperatura - Con color dinámico
            h_stack((
                label(|| "Temp: ".to_string()).style(|s| s.color(Color::from_rgba8(150, 150, 150, 255))),
                label(move || temp_signal.clone().get().to_string())
                    .style(move |s| {
                        let temp = temp_signal.clone().get();
                        let color = if temp > 80 {
                            Color::from_rgba8(255, 50, 50, 255) // Rojo caliente
                        } else if temp > 60 {
                            Color::from_rgba8(255, 165, 0, 255) // Naranja cálido
                        } else {
                            Color::from_rgba8(100, 200, 255, 255) // Azul frío
                        };
                        s.color(color).font_bold()
                    }),
                label(|| "°C".to_string()).style(|s| s.color(Color::from_rgba8(150, 150, 150, 255))),
            ))
            .style(|s| s.gap(5).padding_bottom(5)),
            
            // Driver version
            h_stack((
                label(|| "Driver: ".to_string()).style(|s| s.color(Color::from_rgba8(150, 150, 150, 255))),
                label(move || state.driver_version.get().clone())
                    .style(|s| s.color(Color::from_rgba8(180, 180, 180, 255))),
            ))
            .style(|s| s.gap(5).padding_bottom(5)),
            
            // Botón de actualizar
            Button::new("🔄 Update Driver")
                .action(move || {
                    println!("Updating driver...");
                })
                .style(|s| {
                    s.padding(8)
                        .background(Color::from_rgba8(60, 60, 60, 255))
                        .border_radius(4)
                        .color(Color::from_rgba8(200, 200, 200, 255))
                        .hover(|s| {
                            s.background(Color::from_rgba8(80, 80, 80, 255))
                        })
                })
        ))
        .style(|s| {
            s.padding(12)
                .background(Color::from_rgba8(30, 30, 30, 255))
                .border_radius(8)
                .gap(2)
        })
    ))
    .style(|s| {
        s.width_full()
            .padding(16)
            .gap(10)
            .background(Color::from_rgba8(20, 20, 20, 255))
            .border_radius(10)
            .border(1)
            .border_color(Color::from_rgba8(60, 60, 60, 255))
    })
}

fn body(state: MonitoringState) -> impl IntoView {
    let other_graphics:Vec<GpuInfo> = state.others_graphics.get().clone();
    let mut components = Vec::new();
    components.push(info_card("titulo".to_string(), "descripcion".to_string()));
    for gpu in 1..other_graphics.len() {
        components.push(info_card(
            other_graphics[gpu].name.to_string(),   // Ajusta según tu estructura de datos
            other_graphics[gpu].total_memory.to_string()  // Ajusta según tu estructura de datos
        ));
    };

        
    v_stack((
        gpu_card(state.clone()),
        label(|| "Other GPUs detected".to_string())
            .style(|s| {
                s.font_size(18)
                    .font_bold()
                    .color(Color::from_rgba8(255, 255, 255, 255))
            }),
        h_stack_from_iter(components)   
            .style(|s| {
                s.gap(10)
                    .flex_wrap(FlexWrap::Wrap)
            }),
    ))
    .style(|s| {
        s.width_full()
            .padding(10)
            .gap(10)
            
    })
}

#[derive(Clone)]
pub struct GpuActivity {
    pub name: SyncRwSignal<String>,
    pub usage_percent: SyncRwSignal<Option<f32>>,
    pub vram_used_mb: SyncRwSignal<Option<f32>>,
    pub vram_total_mb: SyncRwSignal<Option<u64>>,
    pub time: SyncRwSignal<String>,
    pub temperatura:SyncRwSignal<u32>,
}

fn footer_actividad(activity: GpuActivity) -> impl View {
    v_stack((
        // Encabezado tipo console
        label(|| String::from("> SYSTEM MONITOR LOG"))
            .style(|s| {
                s.font_size(14)
                    .font_family("Monospace")
                    .font_bold()
                    .color(Color::from_rgba8(0, 255, 0, 255)) // Verde console
                    .padding_bottom(5)
            }),
        
        // Línea de actividad tipo console log
        h_stack((
            label(move || format!("[{}]", activity.time.get().clone()))
                .style(|s| s.color(Color::from_rgba8(100, 100, 100, 255))), // Gris para timestamp
            
            label(move || format!("[{}]", activity.name.get().clone()))
                .style(|s| s.color(Color::from_rgba8(0, 255, 0, 255))), // Verde para nombre
            
            label(move ||{
                let usage = activity.usage_percent.get().clone().unwrap_or(0.0);
                format!("usage:{:.2}%", usage)
            })
                .style(|s| s.color(Color::from_rgba8(255, 255, 0, 255))), // Amarillo para uso
            
            label(move || {
                let vram_gb = activity.vram_total_mb.get().clone().unwrap() as f64 / 1_073_741_824.0;
                format!("RAM:{:.1}GB", vram_gb)
            })
                .style(|s| s.color(Color::from_rgba8(100, 150, 255, 255))), // Azul claro para RAM
            
            label(move || format!("TEMP:{}°C", activity.temperatura.get().clone()))
                .style(move |s| {
                    let temp = activity.temperatura.get().clone();
                    let color = if temp > 80 {
                        Color::from_rgba8(255, 0, 0, 255) // Rojo si está caliente
                    } else if temp > 60 {
                        Color::from_rgba8(255, 165, 0, 255) // Naranja si está cálido
                    } else {
                        Color::from_rgba8(0, 255, 255, 255) // Cian si está frío
                    };
                    s.color(color)
                }),
        ))
        .style(|s| {
            s.gap(8)
                .font_size(12)
                .font_family("Monospace")
                .color(Color::from_rgba8(255, 255, 255, 255))
                .padding(5)
                // .background(Color::from_rgba8(20, 20, 20, 255))
                .border_radius(3)
        }),
    ))
    .style(|s| {
        s.width(100.pct())
            .height(80)
            .padding(10)
            // .background(Color::from_rgba8(10, 10, 10, 255)) // Fondo casi negro
            .border_radius(0)
            .position(Position::Relative)
            .z_index(1000)
            .border_color(Color::from_rgba8(0, 255, 0, 50)) // Línea verde sutil arriba
            .border(1)
    })
}

fn container_view(_id: WindowId) -> impl IntoView {
    let monitoring = MonitoringState {
        gpu_temp: SyncRwSignal::new_sync(1),
        pci_buss:SyncRwSignal::new_sync(String::new()),
        driver_version:SyncRwSignal::new_sync(String::new()),
        gpu_name:SyncRwSignal::new_sync(String::new()),
        gpu_ram:SyncRwSignal::new_sync(0.0),
        gpu_ram_usage:SyncRwSignal::new_sync(0.0),
        others_graphics:SyncRwSignal::new_sync(Vec::new())
    }; 
    
    let activity = GpuActivity {
        name:SyncRwSignal::new_sync(String::new()),
        usage_percent:SyncRwSignal::new_sync(Some(1.0)),
        vram_used_mb:SyncRwSignal::new_sync(Some(1.0)),
        vram_total_mb:SyncRwSignal::new_sync(Some(1)),
        time:SyncRwSignal::new_sync(String::new()),
        temperatura:SyncRwSignal::new_sync(1),
    };

    start_monitoring(monitoring.clone(),activity.clone());
    v_stack((
        header().style(|s| {
            s.width_full()
                .border(1)
                .border_radius(10)
                .border_color(Color::from_rgba8(50, 50, 50, 255))
                .margin(6)
        }),
        body(monitoring.clone()).style(|s| {
            s.width_full()
                .border(1)
                .border_radius(10)
                .border_color(Color::from_rgba8(50, 50, 50, 255))
                .margin(6)
        }),
        footer_actividad(activity.clone()).style(|s|{
            s.width_full()
                .border(1)
                .border_radius(10)
                .border_color(Color::from_rgba8(50, 50, 50, 255))
                .margin(6)
        }),
    ))
    .style(|s| {
        s.width(99.pct())
        // .background(Color::from_rgba8(80, 80, 80,255))
    })
    .scroll()
    .style(|s|{
        s.width_full().height_full()
    })
    
}

fn start_monitoring(state: MonitoringState,activity:GpuActivity) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async {
            loop {
                // Crear un nuevo servicio cada vez (obtiene datos frescos)
                match GpusService::new().await {
                    Ok(gpu_service) => {
                        if let Some(gpu) = gpu_service.get_gpus().first() {
                            let temp = gpu.temperature;
                            state.gpu_temp.update(|v| *v=temp);
                            state.gpu_name.update(|v| *v=gpu.gpu_name_field().unwrap().to_string());
                            state.driver_version.update(|v| *v=gpu.detail.get("Driver Version").unwrap().to_string());
                            state.pci_buss.update(|v| *v=gpu.detail.get("PCI Bus").unwrap().to_string());
                            state.gpu_ram.update(|v| *v = (gpu.total_memory / 1024 / 1024 / 1024) as f32);
                            state.gpu_ram_usage.update(|v| *v= (gpu.used_memory / 1024 / 1024 / 1024) as f32 );
                            let gpus= gpu_service.smi.get_gpu_info().clone();
                            state.others_graphics.update(|v| *v=gpus);
                            let time = current_time_formatted();
                            activity.name.update(|v| *v = gpu.name.clone());
                            let usage = gpu.used_memory.clone();
                            let total = gpu.total_memory.clone();
                            let percent = ((usage as f32 / total as f32) * 100.0).clamp(0.0, 100.0);
                            activity.usage_percent.update(|v| *v = Some(percent));
                            let total_bytes = state.gpu_ram.get().clone();
                            let total_gb = total_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
                            activity.vram_used_mb.update(|v| *v = Some(total_gb));
                            
                            activity.vram_total_mb.update(|v| *v = Some(gpu.total_memory.clone()));
                            activity.time.update(|v| *v = time);
                            activity.temperatura.update(|v| *v = gpu.temperature.clone());
                        } else {
                            println!("⚠️ No hay GPUs");
                            state.gpu_temp.set(0);
                        }
                    },
                    Err(e) => {
                        eprintln!("❌ Error al obtener GPUs: {}", e);
                        state.gpu_temp.set(0);
                    }
                }
                
                
                
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    });
}

fn current_time_formatted() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let seconds_in_day = now % 86400;

    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
}
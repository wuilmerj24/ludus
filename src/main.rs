use std::{io::{BufRead, BufReader}, process::{Command, Stdio}, thread};
use all_smi::device::GpuInfo;
use iced::{Alignment, Border, Color, Element, Length, Renderer, Task, Theme, widget::{Button, Column, Container, Row, Scrollable, Space, button, column, container, responsive, row, rule::horizontal, scrollable, stack, text}, window::{self, Level}};
use sysinfo::System;
use crate::modules::{driver_manager::driver_manager::{DriverAction, DriverManager, DriverStatus, Vendor}, gpu_info::gpu_info::EGpuInfo, system_info::system_info::ESystemInfo};

mod modules;

#[derive(Debug, Clone)]
enum Message {
    MetricasActualizadas{
        cpu_temperature: String,
        gpus: Vec<GpuInfo>,
    },
    GpuAccionPressed(DriverAction,GpuInfo),
    LimpiarLogs,
    ConfirmAlertAction,
    CloseAlert,
}

#[derive(Debug, Clone)]
struct AlertState {
    pub title: String,
    pub message: String,
    pub action: DriverAction,
    pub gpu: GpuInfo,
}

struct LudusUI{
    os_name:String,
    kernel:String,
    cpu:String,
    cpu_temperature: String,
    ram: String,
    gpus: Vec<GpuInfo>,
    logs: Vec<String>,
    active_alert: Option<AlertState>,
}

impl  LudusUI{
    fn new() -> (Self, Task<Message>) {
        let mut system_info = ESystemInfo::new();
        system_info.refresh_all().clone();
        let mut gpu_info = EGpuInfo::new().unwrap();
        let cpu_info = gpu_info.get_all_cpu();
        let mut temp_cpu:String = String::from("0.0 ªC");
        if cpu_info.len() > 0 {
            temp_cpu = cpu_info.get(0).unwrap().temperature.unwrap().to_string();
        }
        let gpus_iniciales = gpu_info.get_all_gpu(false);
        (
            Self {
                os_name: system_info.os_name().clone(),
                kernel: system_info.kernel_version().clone(),
                cpu: system_info.cpu_name().clone(),
                cpu_temperature: temp_cpu.clone(),
                ram:Self::format_bytes(system_info.total_memory()),
                gpus:gpus_iniciales.clone(),
                logs: vec!["[INFO] Sistema de telemetría Ludus iniciado correctamente.".to_string()],
                active_alert:None,
            },
            Task::perform(
                async move {
                    // Al meterlo aquí, Rust lo transforma en un Future<Output = f32>
                    (temp_cpu,gpus_iniciales)
                },
                |(cpu_temp,gpus)| Message::MetricasActualizadas { 
                    cpu_temperature:cpu_temp,
                    gpus
                }
            ),
        )
    }
    
    fn title(&self) -> String {
        let title=env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        format!("{}-{}",title,version)
    }
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MetricasActualizadas { cpu_temperature, gpus } => {
                // 1. Actualizamos el valor de la CPU
                self.cpu_temperature = cpu_temperature;
    
                for nueva_gpu in gpus {
                    if let Some(gpu_actual) = self.gpus.iter_mut().find(|g| g.name == nueva_gpu.name) {
                        gpu_actual.temperature = nueva_gpu.temperature;
                        gpu_actual.utilization = nueva_gpu.utilization;
                        gpu_actual.power_consumption = nueva_gpu.power_consumption;
                        gpu_actual.used_memory = nueva_gpu.used_memory;
                        gpu_actual.frequency = nueva_gpu.frequency;
                        gpu_actual.detail = nueva_gpu.detail;
                    }
                }
                Task::perform(
                    async {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        
                        let mut cpu_temp_actual = String::from("0.0 °C");
                        let mut gpus_frescas = Vec::new();
    
                        if let Ok(mut gpu_info) = EGpuInfo::new() {
                            // Consultar CPU
                            let cpu_info = gpu_info.get_all_cpu();
                            if let Some(cpu) = cpu_info.get(0) {
                                if let Some(temp) = cpu.temperature {
                                    cpu_temp_actual = format!("{:.1} °C", temp);
                                }
                            }
                            // Consultar todas las GPUs del sistema con sus métricas en tiempo real
                            gpus_frescas = gpu_info.get_all_gpu(false);
                        }
    
                        (cpu_temp_actual, gpus_frescas)
                    },
                    |(cpu_temp, gpus_v)| Message::MetricasActualizadas {
                        cpu_temperature: cpu_temp,
                        gpus: gpus_v,
                    }
                )
            },
            Message::GpuAccionPressed(action,gpu_info) => {

                let manager = DriverManager::new();
                let log_msg = format!(
                    "[ACTION] {:?}.",action
                );
                self.logs.push(log_msg);
                let mut vendor:Vendor=Vendor::Unknown;
                
                if gpu_info.clone().name.to_lowercase().contains("nvidia") {
                    vendor = Vendor::Nvidia;
                }

                if gpu_info.clone().name.to_lowercase().contains("amd") {
                    vendor = Vendor::Amd;
                }

                if gpu_info.clone().name.to_lowercase().contains("intel") {
                    vendor = Vendor::Intel;
                }

                match action {
                    DriverAction::Install=>{
                        match manager.build_install_command(&self.os_name, vendor){
                            Ok(mut command) => {
                                // Esto es lo que dispara la ventana de contraseña
                                match command.status() {
                                    Ok(status) if status.success() => println!("Instalado! Reinicia"),
                                    Ok(_) => eprintln!("Usuario canceló o falló la instalación"),
                                    Err(e) => eprintln!("No se encontró pkexec: {}", e),
                                }
                            }
                            Err(e) => eprintln!("{}", e),
                        }
                    }

                    DriverAction::Uninstall=>{
                        self.active_alert = Some(AlertState {
                            title:String::from("Uninstall?"),
                            message:String::from(""),
                            action:DriverAction::Uninstall,
                            gpu:gpu_info.clone()
                        });
                    }

                    DriverAction::Update =>{
                        match manager.build_install_command(&self.os_name, vendor){
                            Ok(mut command) => {
                                // Esto es lo que dispara la ventana de contraseña
                                match command.status() {
                                    Ok(status) if status.success() => println!("Instalado! Reinicia"),
                                    Ok(_) => eprintln!("Usuario canceló o falló la instalación"),
                                    Err(e) => eprintln!("No se encontró pkexec: {}", e),
                                }
                            }
                            Err(e) => eprintln!("{}", e),
                        }
                    },
                }
                Task::none()
            },
            Message::LimpiarLogs => {
                // 👈 Vacía la consola y deja un log limpio
                self.logs.clear();
                self.logs.push("[INFO] Historial de logs reiniciado.".to_string());
                Task::none()
            },
            Message::ConfirmAlertAction => {
                if let Some(alert) = self.active_alert.take() {
                    // Aquí ejecutas la lógica real (instalación / desinstalación)
                    let manager = DriverManager::new();
                    match manager.build_install_command(&self.os_name, Vendor::Amd){
                        Ok(mut command) => {
                            // Esto es lo que dispara la ventana de contraseña
                            match command.status() {
                                Ok(status) if status.success() => println!("Instalado! Reinicia"),
                                Ok(_) => eprintln!("Usuario canceló o falló la instalación"),
                                Err(e) => eprintln!("No se encontró pkexec: {}", e),
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Task::none()
            },
            Message::CloseAlert => {
                self.active_alert = None; // Cerramos el alert
                Task::none()
            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        // 1. GRID HEADER RESPONSIVO
        let header_grid = responsive(|size| {
            let columns = if size.width > 1100.0 {
                4
            } else if size.width > 750.0 {
                3
            } else if size.width > 500.0 {
                2
            } else {
                1
            };

            let static_items: Vec<(String, String)> = vec![
                ("O.S.".to_string(), self.os_name.to_string()),
                ("Kernel".to_string(), self.kernel.to_string()),
                ("CPU".to_string(), self.cpu.to_string()),
                ("Temp CPU".to_string(), self.cpu_temperature.clone()),
                ("Ram".to_string(), self.ram.to_string()),
            ];

            let total_items = static_items.len();

            let mut grid_column = Column::new().spacing(15).width(Length::Fill);
            let mut current_row = Row::new().spacing(15).width(Length::Fill);

            for (index, (title, description)) in static_items.into_iter().enumerate() {
                let i = index + 1;

                let card = container(
                    column![
                        text(title).size(16),
                        text(description).size(12)
                    ]
                    .spacing(6)
                )
                .padding(12)
                .width(Length::FillPortion(1))
                .style(container::bordered_box);

                current_row = current_row.push(card);

                if i % columns == 0 || i == total_items {
                    if i == total_items && total_items % columns != 0 {
                        let faltantes = columns - (total_items % columns);
                        for _ in 0..faltantes {
                            current_row = current_row.push(Space::new());
                        }
                    }

                    grid_column = grid_column.push(current_row);
                    current_row = Row::new().spacing(15).width(Length::Fill);
                }
            }

            grid_column.into()
        });

        // 2. HEADER ZONE
        let header_zone = container(
            column![
                text("System").size(24),
                header_grid
            ]
            .spacing(20)
            .width(Length::Fill)
        )
        .padding(15)
        .width(Length::Fill)
        .style(container::bordered_box);

        // 3. BODY ZONE
        let mut body_items = Column::new().spacing(15).width(Length::Fill);
        let mut selected_gpu = None;
        let mut other_gpus: Row<'_, Message, Theme, Renderer> = Row::new().spacing(10);

        for gpu in &self.gpus {
            if gpu.used_memory > 0 {
                selected_gpu = Some(gpu);
            } else {
                other_gpus = other_gpus.push(
                    container(
                        column![
                            text(&gpu.name).size(18),
                            text("Secondary GPU")
                                .size(13)
                                .color(Color::from_rgb8(140, 140, 140)),
                            horizontal(1),
                            row![
                                text("🌡"),
                                text(format!("{} °C", gpu.temperature)),
                            ]
                            .spacing(6),
                            row![
                                text("💾"),
                                text(format!("{:.1} GB", gpu.total_memory)),
                            ]
                            .spacing(6),
                        ]
                        .spacing(10)
                    )
                    .padding(15)
                    .width(Length::Fixed(220.0))
                    .height(Length::Fixed(140.0))
                    .style(container::bordered_box)
                );
            }
        }

        if let Some(gpu) = selected_gpu {
            let manager = DriverManager::new();
            let status = manager.check_driver_status(&gpu.clone().name, &gpu.clone().detail);
            body_items = body_items.push(
                container(
                    column![
                        row![
                            column![
                                text(&gpu.name).size(28),
                                text("Primary GPU")
                                    .size(15)
                                    .color(Color::from_rgb8(140, 140, 140)),
                            ]
                            .width(Length::Fill),
                            match status {
                                DriverStatus::NotInstalled => {
                                    button(text("Install"))
                                        .padding([8, 16])
                                        .on_press(Message::GpuAccionPressed(DriverAction::Install, gpu.clone()))
                                }
                                DriverStatus::Installed { .. } => {
                                    button(text("Remove"))
                                        .padding([8, 16])
                                        .on_press(Message::GpuAccionPressed(DriverAction::Uninstall, gpu.clone()))
                                }
                                DriverStatus::Unknown(_) => {
                                    button(text("Unknown")).padding([8, 16])
                                }
                            }
                        ]
                        .align_y(Alignment::Center),
                        Space::new(),
                        row![
                            Self::metric("Utilization", format!("{}%", gpu.utilization)),
                            Self::metric("Temperature", format!("{} °C", gpu.temperature)),
                        ]
                        .spacing(10),
                        row![
                            Self::metric(
                                "Memory",
                                format!(
                                    "{} / {}",
                                    Self::format_bytes(gpu.used_memory),
                                    Self::format_bytes(gpu.total_memory)
                                )
                            ),
                            Self::metric("Frequency", format!("{} MHz", gpu.frequency)),
                        ]
                        .spacing(10),
                        row![
                            Self::metric(
                                "Power",
                                format!("{} W", gpu.power_consumption),
                            ),
                            Self::metric(
                                "Clock",
                                gpu.detail
                                    .get("Memory Clock")
                                    .cloned()
                                    .unwrap_or_else(|| "N/A".to_string()),
                            ),
                        ]
                        .spacing(10),
                    ]
                    .spacing(15),
                )
                .padding(20)
                .width(Length::Fill),
            );
        }

        if !self.gpus.is_empty() {
            body_items = body_items
                .push(text("Other GPUs").size(20))
                .push(other_gpus);
        }

        let body_zone = container(body_items)
            .padding(20)
            .width(Length::Fill)
            .style(container::bordered_box);

        // 4. FOOTER ZONE
        let ultimo_log = self.logs.last().cloned().unwrap_or_else(|| "No hay eventos.".to_string());
        let footer_zone = container(
            row![
                text("📋 Logs:")
                    .size(14)
                    .color(Color::from_rgb8(110, 190, 244)),
                text(ultimo_log)
                    .size(14)
                    .color(Color::from_rgb8(200, 200, 200)),
                Space::new(),
                button(text("Limpiar"))
                    .padding([4, 10])
                    .on_press(Message::LimpiarLogs),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        )
        .padding(12)
        .width(Length::Fill)
        .style(container::bordered_box);

        let full_layout = column![
            header_zone,
            body_zone,
            footer_zone
        ]
        .spacing(20)
        .width(Length::Fill)
        .padding(10);

        let main_view: Element<Message> = scrollable(full_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(4.0)
                    .margin(2)
                    .scroller_width(4.0),
            ))
            .into();

        if let Some(alert) = &self.active_alert {
            let alert_box = container(
                column![
                    text(&alert.title).size(20),
                    text(&alert.message).size(15),
                    row![
                        button(text("Cancel"))
                            .padding([8, 16])
                            .on_press(Message::CloseAlert),
                        button(text("Ok"))
                            .style(|_theme, status|{
                                let bg_color = match status {
                                    // Rojo más oscuro cuando pasas el cursor encima
                                    button::Status::Hovered => Color::from_rgb8(185, 28, 28),
                                    // Rojo oscuro al presionar
                                    button::Status::Pressed => Color::from_rgb8(153, 27, 27),
                                    // Rojo principal (rojo brillante de alerta)
                                    _ => Color::from_rgb8(220, 38, 38),
                                };

                                button::Style {
                                    background: Some(bg_color.into()),
                                    text_color: Color::WHITE, // Texto blanco para dar contraste sobre el rojo
                                    border: Border {
                                        radius: 6.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            })
                            .padding([8, 16])
                            .on_press(Message::ConfirmAlertAction),
                    ]
                    .spacing(12)
                ]
                .spacing(15)
            )
            .padding(20)
            .width(400)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb8(35, 35, 35).into()),
                border: iced::Border {
                    color: Color::from_rgb8(70, 70, 70),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            });

            let modal_overlay = container(alert_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.6).into()),
                    ..Default::default()
                });

            stack![
                main_view,
                modal_overlay
            ]
            .into()
        } else {
            main_view
        }
    }
    
    fn format_bytes(bytes: u64) -> String {
        const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    
        if bytes == 0 {
            return "0 B".to_string();
        }
    
        let mut size = bytes as f64;
        let mut unit_index = 0;
    
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
    
        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    fn metric<'a>(
        title: impl Into<String>,
        value: impl Into<String>,
    ) -> Element<'a, Message> {
    
        container(
            column![
                text(title.into())
                    .size(14),
    
                text(value.into())
                    .size(26),
            ]
            .spacing(6)
        )
        .padding(15)
        .width(Length::Fill)
        .style(container::bordered_box)
        .into()
    }
}

fn main() -> iced::Result{
    
    iced::application(
        LudusUI::new, 
        LudusUI::update, 
        LudusUI::view
    )
    .title(LudusUI::title)
    .theme(Theme::Dark)
    .window(iced::window::Settings{
        level:Level::Normal,
        icon: window::icon::from_file("assets/ludus.png").ok(),
        ..Default::default()
    })
    .run()
}



import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";

export interface SoInfo {
  name: string;
  kernel_version: string;
  os_version: string;
}

export interface HardwareInfo {
  name:string;
  pci_ex_info:string;
  ram:string;
  version_driver:string;
  date:string;
}

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  so_name: string = "Desconocido";
  kernel_version: string = "Desconocido";
  os_version: string = "Desconocido";
  estado:boolean = false;
  gpu_name:string = "Desconocida";
  pci_ex_info:string = "Desconocido";
  driver_version:string = "Desconocida";
  constructor(
    
  ){
    setTimeout(() => {
      this.getInfo();
    }, 100);
  }
  
  getInfo(): void {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<any>("get_so_info", {}).then((text: any) => {
      console.log(text);
      if(text){
        this.so_name = text.name;
        this.kernel_version = text.kernel_version;
        this.os_version = text.os_version;
      }
    });
    invoke<Array<HardwareInfo>>("get_hardware_info", {}).then((text: Array<HardwareInfo>) => {
      if(text){
        this.gpu_name = text[0].name;
        this.pci_ex_info = text[0].pci_ex_info;
        this.driver_version = text[0].version_driver;
        console.log(text);
      }
    });
  }
}

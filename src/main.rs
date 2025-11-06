mod ui;

use slint::{ComponentHandle, Timer, TimerMode, PhysicalPosition, WindowPosition};
use slint::winit_030::WinitWindowAccessor;
use ui::{MainWindow, SplashScreen};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    // Criar splash screen
    let splash = SplashScreen::new()?;
    
    // Configurar splash para versão de desenvolvimento
    splash.set_is_dev_version(true);
    splash.set_version("v0.1.0 - Master".into());
    splash.set_message("Inicializando...".into());
    
    // Mostrar splash
    splash.show()?;
    
    // Centralizar a janela após um pequeno delay para garantir que está totalmente inicializada
    let splash_weak_for_center = splash.as_weak();
    let center_timer = Timer::default();
    center_timer.start(TimerMode::SingleShot, std::time::Duration::from_millis(10), move || {
        if let Some(splash) = splash_weak_for_center.upgrade() {
            let window = splash.window();
            let size = window.size();
            
            // Tentar obter as dimensões da tela através do winit
            window.with_winit_window(|winit_window| {
                if let Some(monitor) = winit_window.current_monitor() {
                    let monitor_size = monitor.size();
                    let scale_factor = monitor.scale_factor();
                    
                    // Calcular o centro da tela considerando o DPI scaling
                    let screen_width = (monitor_size.width as f64 / scale_factor) as i32;
                    let screen_height = (monitor_size.height as f64 / scale_factor) as i32;
                    
                    let x = (screen_width - size.width as i32) / 2;
                    let y = (screen_height - size.height as i32) / 2;
                    
                    window.set_position(WindowPosition::Physical(PhysicalPosition::new(x.max(0), y.max(0))));
                }
            });
        }
    });
    
    // Usar timer para sequência de mensagens
    let splash_weak = splash.as_weak();
    let phase = Rc::new(RefCell::new(0));
    
    let timer = Timer::default();
    let phase_clone = phase.clone();
    
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(300), move || {
        let mut current_phase = phase_clone.borrow_mut();
        
        if let Some(splash) = splash_weak.upgrade() {
            match *current_phase {
                0 => splash.set_message("Verificando banco de dados...".into()),
                1 => splash.set_message("Lendo configurações...".into()),
                2 => splash.set_message("Configurando interface...".into()),
                3 => splash.set_message("Carregando recursos...".into()),
                4 => splash.set_message("Inicializando canvas...".into()),
                5 => splash.set_message("Preparando ferramentas...".into()),
                6 => {
                    // Última fase: fechar splash - a janela principal será criada depois
                    splash.set_message("Abrindo aplicação...".into());
                    let _ = splash.hide();
                }
                _ => {}
            }
            *current_phase += 1;
        }
    });
    
    // Executar o loop de eventos da splash
    splash.run()?;
    
    // Pequeno delay para garantir que a splash fechou completamente
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Após a splash fechar, criar e executar a janela principal
    let main_window = MainWindow::new()?;
    
    // Configurar callback para sair da aplicação
    let main_window_weak = main_window.as_weak();
    main_window.on_quit_app(move || {
        if let Some(window) = main_window_weak.upgrade() {
            window.hide().unwrap();
            // Força o encerramento do loop de eventos
            std::process::exit(0);
        }
    });
    
    // Forçar a exibição da janela
    main_window.show()?;
    
    // Centralizar a janela principal após um pequeno delay
    let main_weak_for_center = main_window.as_weak();
    let main_center_timer = Timer::default();
    main_center_timer.start(TimerMode::SingleShot, std::time::Duration::from_millis(10), move || {
        if let Some(main_win) = main_weak_for_center.upgrade() {
            let window = main_win.window();
            let size = window.size();
            
            // Centralizar e forçar foco no Windows
            window.with_winit_window(|winit_window| {
                if let Some(monitor) = winit_window.current_monitor() {
                    let monitor_size = monitor.size();
                    let scale_factor = monitor.scale_factor();
                    
                    // Calcular o centro da tela considerando o DPI scaling
                    let screen_width = (monitor_size.width as f64 / scale_factor) as i32;
                    let screen_height = (monitor_size.height as f64 / scale_factor) as i32;
                    
                    let x = (screen_width - size.width as i32) / 2;
                    let y = (screen_height - size.height as i32) / 2;
                    
                    window.set_position(WindowPosition::Physical(PhysicalPosition::new(x.max(0), y.max(0))));
                }
                
                // Forçar foco na janela (importante no Windows)
                winit_window.focus_window();
            });
        }
    });
    
    main_window.run()
}

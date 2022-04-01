// #[path = "./rusty_system/basic_sys.rs"]
// mod basic_sys;

// #[path = "./rusty_system/basic_process.rs"]
// mod basic_process;

#[path = "./rusty_system/basic_network.rs"]
mod basic_network;

// use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout};
use cursive::Cursive;
use cursive::view::Resizable;
use sysinfo::{System, SystemExt};
// use sysinfo::{SystemExt};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );

    // Starts the event loop.
    siv.run();
}

fn information(s: &mut Cursive) {
    
    let mut sys = System::new_all();

    s.pop_layer();
    s.add_layer(
        Dialog::text("Informations User")
            .title("RustyMinitel / Informations")
            .button("Return Menu", menu),
    );
}

fn network(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Network User :")
            .title("RustyMinitel / Network")
            .button("Return Menu", menu)
            .biblionetwork()
            .fixed_size((75, 25)),
        );
}

fn biblionetwork (s: &mut Cursive) {

    let mut cpt = 1;
    let mut sys = System::new_all();

    sys.refresh_all();
    let networks = basic_network::get_networks(&sys);

    Dialog::text("\n\n ============== NETWORK INFO ===============");
        for net in networks{
            println!("\nNetwork n°{} : ",cpt);
            for(key,value) in net.into_iter(){
                print!("{} : {} ",key,value);
            }
            cpt+=1;
        } cpt = 0;
    }

fn process(s: &mut Cursive) {
    // let select = SelectView::<String>::new()
    //     .on_submit(on_submit)
    //     .with_name("select")
    //     .fixed_size((10, 5));

    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::horizontal()).title("Selection "));
}

fn menu(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );
}

// fn on_submit(s: &mut Cursive, name: &str) {
//     s.pop_layer();
//     s.add_layer(
//         Dialog::text(format!("Name: {}\nAwesome: yes", name))
//             .title(format!("{}'s info", name))
//             .button("Quit", Cursive::quit),
//     );
// }

// fn main() {
//     let mut sys = System::new_all();
//     let cpu = basic_sys::get_basic_cpu_infos(&sys);
//     let adv_cpu = basic_sys::get_adv_cpu_infos(&sys);
//     let base = basic_sys::get_os_infos(&sys);
//     let processes = basic_process::get_all_process(&sys);
//     let networks = basic_network::get_networks(&sys);

//     let mut cpt = 1;

//     sys.refresh_all();

//     println!("\n============== CPU BASE INFO ===============");
//     for (key, value) in cpu.into_iter() {
//         println!("{} : {}", key, value);

//     }
//     println!("\n============== CPU FULL INFO ===============");
//     for (key, value) in adv_cpu.into_iter() {
//         println!("{} : ",key);
//         if key == "core_temps" {
//             for i in value {
//                 print!("Core {} : {}°C ",cpt,i);
//             }
//             cpt = 1;
//         }else if key == "core_freqs"{
//             for i in value {
//                 print!("Core {} : {}MHz ",cpt,i);
//                 cpt+=1;
//             }
//             cpt = 1;
//         }else if key=="comp_temps"{
//             for i in value {
//                 print!("Component : {}°C ",i);

//             }
//         }
//         cpt = 1;

//     }
//     println!("\n============== SYS BASE INFO ===============");

//     for(key,value) in base.into_iter(){
//         println!("{} : {:?}", key, &value);
//     }

//     println!("\n\n ============== PROCESSES INFO ===============");
//     for proc in processes{
//         println!("\nProcess n°{} : ",cpt);
//         for(key,value) in proc.into_iter(){
//             print!("{} : {} ",key,value);
//         }
//         cpt+=1;
//     } cpt = 0;

//     println!("\n\n ============== NETWORK INFO ===============");
//     for net in networks{
//         println!("\nNetwork n°{} : ",cpt);
//         for(key,value) in net.into_iter(){
//             print!("{} : {} ",key,value);
//         }
//         cpt+=1;
//     } cpt = 0;

//     println!("\n\n ============== TABLE ROUTES IP ===============");
//     println!("{}",basic_network::get_ip_routes());

// }

use slint::VecModel;

slint::include_modules!();

fn main() {
    let ui: MainWindow = MainWindow::new().unwrap();

    let mut runners = Vec::<Value>::new();

    runners.push(Value {
        name: "yooo".into()
    });

    let model = slint::ModelRc::new(VecModel::from(runners));

    ui.set_runners(
        model
    );

    ui.on_save(|values| {
        println!("Values: {:?}", values);
        std::process::exit(0);
    });
    
    ui.run().unwrap();
}
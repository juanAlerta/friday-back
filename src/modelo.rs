use bard-rs::Bard;

pub mod modulo1 {
    pub fn modelo(){

        let model = Bard::new();
        let text = model.generate("¿Cómo puedo usar Bard desde una función en Rust?");
        println!("{}", text);
        let text = model.generate("Escribeme que crees que hace este programa?.");
        println!("{}", text);
    
    }    
}

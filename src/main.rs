use rust_bert::gpt2::Gpt2Generator;
use rust_bert::pipelines::generation::GPT2GeneratorOutput;
use rust_bert::pipelines::generation::LanguageGenerator;

fn main() -> anyhow::Result<()> {
    // Cargar el modelo pre-entrenado
    let model = Gpt2Generator::new(Default::default())?;

    // Ejemplo de instrucción compleja en lenguaje humano
    let input_text = "Dibuja un círculo azul en el centro de la pantalla.";

    // Generar instrucciones más simples
    let output: GPT2GeneratorOutput = model.generate(Some(input_text), None);

    // Imprimir las instrucciones generadas
    println!("{}", output.text);

    Ok(())
} 
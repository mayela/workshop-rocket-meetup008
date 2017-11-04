#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;

extern crate rocket;

use rand::Rng;

const FRASES: &[&'static str] = &[
    "“La fuerza estará ya contigo… siempre.” (Obi-Wan Kenobi a Luke Skywalker en la Estrella de la Muerte. STAR WARS, episodio IV, Una Nueva Esperanza)",
    "“Miedo, ira, agresividad, el lado oscuro ellos son. Si algún día rigen tu vida, para siempre tu destino dominarán.” (Yoda a Luke Skywalker. STAR WARS, episodio V, El Imperio Contrataca)",
    "“¡Tú siempre con tus “NO PUEDE HACERSE”! ¿Es que escuchándome no estabas?.” (Yoda a Luke Skywalker. STAR WARS, episodio V, El Imperio Contrataca)",
    "“Impresionante… ¡Muy impresionante! Controlas tu miedo, Obi-Wan te ha instruido bien…” (Darth Vader a Luke Skywalker, durante el duelo de sables de luz. STAR WARS, episodio V, El Imperio Contrataca)",
];

fn sacar_frase_aleatoria(frases: &[&'static str]) -> &'static str {
    let mut generador_aleatorio = rand::thread_rng();
    let indice = generador_aleatorio.gen_range(0, FRASES.len());

    frases.get(indice).expect("No se pudo obtener una frase")
}

#[get("/")]
fn index() -> String {
    let frase = sacar_frase_aleatoria(&FRASES);
    frase.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

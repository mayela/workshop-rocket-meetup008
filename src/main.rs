#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rand::Rng;
use rocket::request::Form;
use rocket_contrib::{Template, Json};

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

#[derive(Serialize)]
struct ContextoPlantilla {
    frase: String,
    nombre: Option<String>,
}

#[get("/")]
fn index() -> Template {
    let contexto = ContextoPlantilla {
        frase: sacar_frase_aleatoria(&FRASES).to_string(),
        nombre: None,
    };
    Template::render("index", &contexto)
}

#[derive(FromForm)]
struct Formulario {
    nombre: String,
}

#[post("/", data = "<formulario>")]
fn index_personalizado(formulario: Form<Formulario>) -> Template {
    let form = formulario.get();
    let nombre = form.nombre.clone();
    let contexto = ContextoPlantilla {
        frase: sacar_frase_aleatoria(&FRASES).to_string(),
        nombre: Some(nombre),
    };

    Template::render("index", &contexto)
}

#[get("/frases")]
fn api_frases() -> Json<&'static [&'static str]> {
    Json(FRASES)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, index_personalizado])
        .mount("/api/v1", routes![api_frases])
        .attach(Template::fairing())
        .launch();
}

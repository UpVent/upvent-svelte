use rocket::serde::Serialize;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Testimonial {
    /// Obligatory field ID
    id: i32,
    /// The name of the person giving their testimonial.
    name: String,
    /// The testimonial itself.
    testimonial: String,
    /// The job or title of the person giving their testimonail.
    workplace: String,
    /// Website where the person works or shows their information.
    website: String,
}

/// Stores a single instance of a project used in the portfolio section in
/// this site.
/// (About Us)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Project {
    /// Obligatory field ID
    id: i32,
    /// The title or name of the project
    title: String,
    /// The web address where the project is stored
    site: String,
    /// The project description. This is shown in a card, try to be brief.
    description: String,
}

use rocket::serde::Serialize;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Testimonial {
    id: i32,
    name: String,
    testimonial: String,
    workplace: String,
    website: String,
}

/// Stores a single instance of a project used in the portfolio section in
/// this site.
/// (About Us)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Project {
    id: i32,
    title: String,
    site: String,
    description: String,
}

/// Stores a single Free Software Project shown in the "services" page
/// of this site.
/// (Services)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct FSProject {
    id: i32,
    title: String,
    description: String,
    github_addr: String,
    support_addr: String,
    proj_license: String,
    license_link: String,
}

/// Stores a single License text shown in the "licenses" page
/// of this site.
/// (Licenses)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct License {
    id: i32,
    name: String,
    verbatim: String,
    license_link: String,
}

/// Stores a single Hall Of Fame shown in the "licenses" page
/// of this site.
/// (Licenses)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct HOF {
    id: i32,
    name: String,
}

/// Stores a single team member shown in the "team" page
/// of this site. The is_collab field is a boolean used to indicate if the
/// team member is an outside collaborator. The status is pretty simple.
///
/// true = Outside Collaborator
/// false = UpVent Member
/// (Team)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct TeamMember {
    id: i32,
    name: String,
    position: String,
    is_collab: bool,
}

/// Stores a single and unique privacy policy to make this site compliant with
/// inside / outside country privacy laws (GDPR and others).
/// (Privacy Policy)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct PrivacyPolicy {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

/// Stores a single and unique terms of service + refund policy to make this
/// site compliant with inside / outside country trade laws.
/// (Terms Of Service)
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct TermsOfService {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

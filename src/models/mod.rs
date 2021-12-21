use rocket::serde::Serialize;
use uuid::Uuid;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Testimonial {
    id: Uuid,
    name: String,
    testimonial: String,
    workplace: String,
    website: String,
}

impl Testimonial {
    /// Returns a Testimonial with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    ///
    /// ```
    /// // Clone an existing Testimonial
    /// let testimonial = Testimonial::clone();
    /// ```

    pub fn clone(&self) -> Testimonial {
        Testimonial {
            id: self.id,
            name: self.name.clone(),
            testimonial: self.testimonial.clone(),
            workplace: self.workplace.clone(),
            website: self.workplace.clone(),
        }
    }
}

/// Stores a single instance of a project used in the portfolio section in
/// this site.
/// (About Us)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Project {
    id: Uuid,
    title: String,
    site: String,
    description: String,
}

impl Project {
    /// Returns a Project with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Testimonial
    /// let project = Project::clone();
    /// ```

    pub fn clone(&self) -> Project {
        Project {
            id: self.id,
            title: self.title.clone(),
            site: self.site.clone(),
            description: self.description.clone(),
        }
    }
}

/// Stores a single Free Software Project shown in the "services" page
/// of this site.
/// (Services)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct FSProject {
    id: Uuid,
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
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct License {
    id: Uuid,
    name: String,
    verbatim: String,
    license_link: String,
}

/// Stores a single Hall Of Fame shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HOF {
    id: Uuid,
    name: String,
}

/// Stores a single team member shown in the "team" page
/// of this site. The is_collab field is a boolean used to indicate if the
/// team member is an outside collaborator. The status is pretty simple.
///
/// true = Outside Collaborator
/// false = UpVent Member
/// (Team)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TeamMember {
    id: Uuid,
    name: String,
    position: String,
    is_collab: bool,
}

/// Stores a single and unique privacy policy to make this site compliant with
/// inside / outside country privacy laws (GDPR and others).
/// (Privacy Policy)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct PrivacyPolicy {
    id: Uuid,
    title: String,
    changelog: String,
    text: String,
}

/// Stores a single and unique terms of service + refund policy to make this
/// site compliant with inside / outside country trade laws.
/// (Terms Of Service)
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TermsOfService {
    id: Uuid,
    title: String,
    changelog: String,
    text: String,
}

// ===== Blog page =====

/// Stores a single blog post.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    id: Uuid,
    published: bool,
    title: String,
    description: String,
    category: String,
    content: String,
}

// ===== Marketcloud page =====

/// Stores a single marketplace product
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: Uuid,
    name: String,
    price: f64,
    category: String,
    apptype: String,
    short_description: String,
    description: String,
    stripe_link: String,
    available: bool,
}

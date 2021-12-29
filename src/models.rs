use rocket::serde::uuid::Uuid;
use rocket::serde::Serialize;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[derive(Serialize, Queryable)]
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
#[derive(Serialize, Queryable)]
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
#[derive(Serialize, Queryable)]
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

impl FSProject {
    /// Returns a Free Software Project with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Free Software Project
    /// let fsproject = FSProject::clone();
    /// ```

    pub fn clone(&self) -> FSProject {
        FSProject {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            github_addr: self.github_addr.clone(),
            support_addr: self.support_addr.clone(),
            proj_license: self.proj_license.clone(),
            license_link: self.license_link.clone(),
        }
    }
}

/// Stores a single License text shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct License {
    id: Uuid,
    name: String,
    verbatim: String,
    license_link: String,
}

impl License {
    /// Returns a License Text with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing License
    /// let license = License::clone();
    /// ```

    pub fn clone(&self) -> License {
        License {
            id: self.id,
            name: self.name.clone(),
            verbatim: self.verbatim.clone(),
            license_link: self.license_link.clone(),
        }
    }
}

/// Stores a single Hall Of Fame shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct HOF {
    id: Uuid,
    name: String,
}

impl HOF {
    /// Returns a Hall Of Fame item with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Hall Of Fame item
    /// let famous_project = HOF::clone();

    pub fn clone(&self) -> HOF {
        HOF {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

/// Stores a single team member shown in the "team" page
/// of this site. The is_collab field is a boolean used to indicate if the
/// team member is an outside collaborator. The status is pretty simple.
///
/// true = Outside Collaborator
/// false = UpVent Member
/// (Team)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct TeamMember {
    id: Uuid,
    name: String,
    position: String,
    is_collab: bool,
}

impl TeamMember {
    /// Returns a Team Member with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Team Member
    /// let member = TeamMember::clone();

    pub fn clone(&self) -> TeamMember {
        TeamMember {
            id: self.id,
            name: self.name.clone(),
            position: self.position.clone(),
            is_collab: self.is_collab,
        }
    }
}

/// Stores a single and unique privacy policy to make this site compliant with
/// inside / outside country privacy laws (GDPR and others).
/// (Privacy Policy)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct PrivacyPolicy {
    id: Uuid,
    title: String,
    changelog: String,
    text: String,
}

impl PrivacyPolicy {
    /// Returns the Privacy Policy with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Privacy Policy
    /// let p_policy = PrivacyPolicy::clone();

    pub fn clone(&self) -> PrivacyPolicy {
        PrivacyPolicy {
            id: self.id,
            title: self.title.clone(),
            changelog: self.changelog.clone(),
            text: self.text.clone(),
        }
    }
}

/// Stores a single and unique terms of service + refund policy to make this
/// site compliant with inside / outside country trade laws.
/// (Terms Of Service)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct TermsOfService {
    id: Uuid,
    title: String,
    changelog: String,
    text: String,
}

impl TermsOfService {
    /// Returns the Terms Of Service with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Terms Of Service item
    /// let tos_policy = TermsOfService::clone();

    pub fn clone(&self) -> TermsOfService {
        TermsOfService {
            id: self.id,
            title: self.title.clone(),
            changelog: self.changelog.clone(),
            text: self.text.clone(),
        }
    }
}

// ===== Blog page =====

/// Stores a single blog post.
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct Post {
    id: Uuid,
    published: bool,
    title: String,
    description: String,
    category: String,
    content: String,
}

impl Post {
    /// Returns the Post with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Post item
    /// let post = Post::clone();

    pub fn clone(&self) -> Post {
        Post {
            id: self.id,
            published: self.published,
            title: self.title.clone(),
            description: self.description.clone(),
            category: self.category.clone(),
            content: self.content.clone(),
        }
    }
}

// ===== Marketcloud page =====

/// Stores a single marketplace product
#[derive(Serialize, Queryable)]
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

impl Product {
    /// Returns the Product with the id given to it.
    ///
    /// # Arguments
    ///
    /// * `none` - No arguments implemented yet for this struct.
    ///
    /// # Examples
    /// ```
    /// // Clone an existing Product item
    /// let product = Product::clone();

    pub fn clone(&self) -> Product {
        Product {
            id: self.id,
            name: self.name.clone(),
            price: self.price,
            category: self.category.clone(),
            apptype: self.apptype.clone(),
            short_description: self.short_description.clone(),
            description: self.description.clone(),
            stripe_link: self.stripe_link.clone(),
            available: self.available,
        }
    }
}

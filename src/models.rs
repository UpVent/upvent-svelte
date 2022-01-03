use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/* Testimonial Model related imports */
use crate::schema::testimonials;
use crate::schema::testimonials::dsl::testimonials as all_testimonials;

/* Project Model related imports */
use crate::schema::projects;
use crate::schema::projects::dsl::projects as all_projects;

/* Free Software Model related imports */
use crate::schema::fsprojects;
use crate::schema::fsprojects::dsl::fsprojects as all_fsprojects;

/* License Model related imports */
use crate::schema::licenses;
use crate::schema::licenses::dsl::licenses as all_licenses;

/* Hall Of Fame Model related imports */
use crate::schema::hofs;
use crate::schema::hofs::dsl::hofs as all_hofs;

/* Team Member Model related imports */
use crate::schema::teammembers;
use crate::schema::teammembers::dsl::teammembers as all_teammembers;

/* Privacy Policy Model related imports */
use crate::schema::privacypolicies;
use crate::schema::privacypolicies::dsl::privacypolicies as all_privacypolicies;

/* Terms Of Service Model related imports */
use crate::schema::termsofservices;
use crate::schema::termsofservices::dsl::termsofservices as all_termsofservices;

/* Blog Post Model related imports */
use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;

/* Marketcloud product related imports */
use crate::schema::products;
use crate::schema::products::dsl::products as all_products;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Testimonial {
    pub id: i32,
    pub name: String,
    pub testimonial: String,
    pub workplace: String,
    pub website: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "testimonials"]
pub struct NewTestimonial {
    pub name: String,
    pub testimonial: String,
    pub workplace: String,
    pub website: String,
}

impl Testimonial {
    /// Returns a vector consisting of a single Testimonial in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The testimonial ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first testimonial saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let first = Testimonial::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .find(id)
            .load::<Testimonial>(conn)
            .expect("Error loading testimonial")
    }

    /// Returns a vector of all Testimonials saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first testimonial saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let test_list = Testimonial::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .order(testimonials::id.desc())
            .load::<Testimonial>(conn)
            .expect("Error loading testimonials")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, test: NewTestimonial) -> bool {
        use crate::schema::testimonials::dsl::{
            name as n, testimonial as t, website as ws, workplace as w,
        };

        let NewTestimonial {
            name,
            testimonial,
            workplace,
            website,
        } = test;

        diesel::update(all_testimonials.find(id))
            .set((
                n.eq(name),
                t.eq(testimonial),
                w.eq(workplace),
                ws.eq(website),
            ))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * test: A `NewTestimonial` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the Testimonials table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let testimonial = models::NewTestimonial {
    ///    name: String::from("VentGrey"),
    ///    testimonial: String::from("This testimonial text is a test. The cake is a lie!"),
    ///    workplace: String::from("UpVent Technologies"),
    ///    website: String::from("https://upvent.codes/"),
    /// };
    ///
    /// if models::Testimonial::insert(testimonial, &conn) {
    ///    println!("Testimonial inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the testimonial!");
    /// }
    /// ```
    ///
    pub fn insert(test: NewTestimonial, conn: &SqliteConnection) -> bool {
        diesel::insert_into(testimonials::table)
            .values(&test)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if Testimonial::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_testimonials.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .filter(testimonials::name.eq(name))
            .load::<Testimonial>(conn)
            .expect("Error loading testimonials by name")
    }
}

/// Stores a single instance of a project used in the portfolio section in
/// this site.
/// (About Us)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Project {
    id: i32,
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
pub struct FSProject {
    id: i32,
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
pub struct License {
    id: i32,
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
pub struct HOF {
    id: i32,
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
pub struct TeamMember {
    id: i32,
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
pub struct PrivacyPolicy {
    id: i32,
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
pub struct TermsOfService {
    id: i32,
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
pub struct Post {
    id: i32,
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
pub struct Product {
    id: i32,
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

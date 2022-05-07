use tagfilter_lang::Requirement;

struct Item {
    description: &'static str,
    tags: &'static [&'static str],
    filename: &'static str,
}

struct Db {
    items: &'static [Item],
}

macro_rules! item {
    ($filename:literal $desc:literal: $($tag:literal)*) => {
        Item {
            description: $desc,
            tags: &[$($tag,)*],
            filename: $filename,
        }
    };
}

impl Db {
    fn query<'src: 'a, 'a>(&'a self, text: &'src str) -> impl Iterator<Item = &'a Item> + 'a {
        let reqs = tagfilter_lang::parse(text).unwrap();
        self.items
            .iter()
            .filter(move |item| reqs.iter().all(|req| req_matches(req, item)))
    }
}

fn req_matches(req: &Requirement, item: &Item) -> bool {
    match req {
        Requirement::Tag(name) => item.tags.contains(name),
        Requirement::FnCall(call) => match call.name {
            "any" => call.params.iter().any(|req| req_matches(req, item)),
            "file" => call.params.get(0).map_or(false, |param| match param {
                Requirement::Tag(name) => item.filename.contains(name),
                _ => false,
            }),
            "notags" => item.tags.is_empty(),
            _ => todo!(),
        },
        Requirement::Not(req) => !req_matches(req, item),
    }
}

impl Default for Db {
    fn default() -> Self {
        Self {
            items: &[
                item!("cat_hat.jpg" "A cat with a hat": "cat" "hat"),
                item!("cat_dog.jpg" "A cat with a dog": "cat" "dog"),
                item!("dog_hat.jpg" "A dog with a hat": "hat" "dog"),
                item!("cat_dog_hats.jpg" "A cat and dog wearing hats": "cat" "dog" "hat"),
                item!("cat_dog_foresthats.png" "A cat and dog wearing hats in a forest": "cat" "dog" "hat" "forest"),
                item!("forestcat.gif" "A cat in a forest": "cat" "forest"),
                item!("doginaforest.jpg" "A dog in a forest": "dog" "forest"),
                item!("newimage.png" "Some new image that hasn't been tagged yet":),
            ],
        }
    }
}

fn main() {
    let query = std::env::args().nth(1).unwrap_or_default();
    println!("Query: '{}'", query);
    let db = Db::default();
    for item in db.query(&query) {
        println!("{}: {}", item.filename, item.description);
    }
}

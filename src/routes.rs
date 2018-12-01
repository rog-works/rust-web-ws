use rocket_contrib::Json;
use models::Device;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/devices")]
fn devices() -> Json<Vec<Device>> {
	Json(vec![Device {
		id: 1,
		name: "light".into(),
		stat: true,
	}])
}

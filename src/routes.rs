use rocket_contrib::json::Json;
use models::Device;

#[get("/")]
pub fn index() -> &'static str {
	"Hello, world!"
}

#[get("/devices")]
pub fn devices() -> Json<Vec<Device>> {
	Json(vec![Device {
		id: 1,
		name: "light".into(),
		stat: true,
	}])
}

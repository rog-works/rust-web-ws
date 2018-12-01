#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub id: u32,
	pub name: String,
	pub stat: bool,
}

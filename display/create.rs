use block_tools::{Error, display_api::{CreationObject, component::{atomic::text::TextComponent, form::input::InputComponent}}};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_create_display() -> Result<CreationObject, Error> {
		let header = TextComponent::heading("New Data Block");

		let main = InputComponent {
			label: Some("Data".to_string()),
			name: Some("DATA".to_string()),
			..Default::default()
		};

		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: "$[DATA]$".into(),
		};
		Ok(object)
	}
}

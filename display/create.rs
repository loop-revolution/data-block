use block_tools::{
	display_api::{
		component::{
			input::InputComponent,
			text::{TextComponent, TextPreset},
		},
		CreationObject,
	},
	Error,
};

use crate::blocks::data_block::DataBlock;

impl DataBlock {
	pub fn handle_create_display() -> Result<CreationObject, Error> {
		let header = TextComponent::new("New Data Block").preset(TextPreset::Heading);
		let main = InputComponent::new().label("Data").name("DATA");
		let object = CreationObject {
			header_component: box header,
			main_component: box main,
			input_template: "$[DATA]$".into(),
		};
		Ok(object)
	}
}

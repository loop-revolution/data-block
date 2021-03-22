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

pub fn create_display() -> Result<CreationObject, Error> {
	let header = TextComponent::new("New Data Block").preset(TextPreset::Heading);
	let main = InputComponent::new().label("Data").name("DATA");
	let object = CreationObject {
		header_component: Box::new(header),
		main_component: Box::new(main),
		input_template: "$[DATA]$".into(),
	};
	Ok(object)
}

use crate::blocks::data_block::DataBlock;
use block_tools::blocks::BlockType;
use block_tools::display_api::{
	component::{
		input::InputComponent,
		text::{TextComponent, TextPreset},
	},
	MethodObject,
};

impl DataBlock {
	/// A re-usable masked data input
	pub fn masked_editable_data(
		block_id: String,
		value: Option<String>,
		heading: bool,
	) -> InputComponent {
		let name = format!("DATA{}", block_id);
		let method = MethodObject {
			block_type: DataBlock::name(),
			arg_template: format!("$[{}]$", name),
			block_id,
			method_name: "edit".into(),
		};
		let input = InputComponent::new()
			.name(&name)
			.with_confirm(method.into());
		if let Some(value) = value {
			let mut mask = TextComponent::new(value.clone());
			if heading {
				mask = mask.preset(TextPreset::Heading)
			}
			input.initial_value(&value).mask(mask)
		} else {
			input
		}
	}
}

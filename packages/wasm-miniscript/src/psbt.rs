use miniscript::bitcoin::Psbt;
use miniscript::psbt::PsbtExt;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError};
use crate::descriptor::WrapDescriptorEnum;
use crate::WrapDescriptor;

#[wasm_bindgen]
pub struct WrapPsbt(Psbt);

#[wasm_bindgen()]
impl WrapPsbt {
    pub fn deserialize(psbt: Vec<u8>) -> Result<WrapPsbt, JsError> {
        Ok(WrapPsbt(Psbt::deserialize(&psbt).map_err(JsError::from)?))
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    #[wasm_bindgen(js_name = updateInputWithDescriptor)]
    pub fn update_input_with_descriptor(&mut self, input_index: usize, descriptor: WrapDescriptor) -> Result<(), JsError> {
        match descriptor.0 {
            WrapDescriptorEnum::Definite(d) => {
                self.0.update_input_with_descriptor(input_index, &d).map_err(JsError::from)
            }
            WrapDescriptorEnum::Derivable(_, _) => {
                Err(JsError::new("Cannot update input with a derivable descriptor"))
            }
            WrapDescriptorEnum::String(_) => {
                Err(JsError::new("Cannot update input with a string descriptor"))
            }
        }
    }
}

use super::common_arrays::*;
use super::common::*;
use super::{
    ABIParameter, 
    DeserializationError
};

use tonlabs_sdk_emulator::stack::{
    BuilderData,
    SliceData
};
use tonlabs_sdk_emulator::bitstring::{Bit, Bitstring};

// put dynamic array to chain or to separate branch depending on array size
pub fn prepend_dynamic_array<T: ABIParameter>(
    mut destination: BuilderData,
    array: &[T]
) -> BuilderData {

    let mut array_size = 0;
    for i in array {
        array_size += i.get_in_cell_size();
    }

    // if array doesn't fit into one cell, we put into separate chain
    // Note: Since length is one byte value any array longer than 256 
    // must be written into a separate cell.
    if array.len() > 256 || array_size > destination.bits_capacity() {
        destination = put_array_to_separate_branch(destination, array);
    } else {
        // if array fit into cell data, put in into main chain
        destination = prepend_array_items_to_chain(destination, array);

        let mut bitstring = Bitstring::new();
        bitstring.append_bit(&Bit::One);
        bitstring.append_bit(&Bit::Zero);
        bitstring.append_u8(array.len() as u8);

        destination = prepend_data_to_chain(destination, bitstring);
    }

    destination
}

impl<T> ABIParameter for &[T]
where
    T: ABIParameter,
{
    fn prepend_to(&self, destination: BuilderData) -> BuilderData {
        prepend_dynamic_array(destination, self)
    }

    fn type_signature() -> String {
        format!("{}[]", T::type_signature())
    }

    fn get_in_cell_size(&self) -> usize {
        let mut result = 8;
        for i in *self {
            result += i.get_in_cell_size();
        }

        // if array doesn't fit into cell it is put in separate chain and only 2 bits are put in main chain cell
        if result > BuilderData::new().bits_capacity() {
            2
        } else {
            result + 2
        }
    }

    fn read_from(cursor: SliceData) -> Result<(Self, SliceData), DeserializationError> {
        unimplemented!();
    }

}

impl<T> ABIParameter for Vec<T>
where
    T: ABIParameter,
{
    fn prepend_to(&self, destination: BuilderData) -> BuilderData {
        prepend_dynamic_array(destination, self.as_slice())
    }

    fn type_signature() -> String {
        format!("{}[]", T::type_signature())
    }

    fn get_in_cell_size(&self) -> usize {
        let mut result = 8;
        for i in self {
            result += i.get_in_cell_size();
        }

        println!("inner size {}", result);
        
        // if array doesn't fit into cell it is put in separate chain and only 2 bits are put in main chain cell
        if result > BuilderData::new().bits_capacity() {
            2
        } else {
            result + 2
        }
    }

    fn read_from(cursor: SliceData) -> Result<(Self, SliceData), DeserializationError> {
        unimplemented!();
    }
}
use burn::{
    nn::{Linear, LinearConfig, Relu}, 
    prelude::{Backend, Module, Tensor}, 
    backend::NdArray,
};
use crate::state::kryds::GameState;

pub type B = NdArray<f32>;

impl PartialEq for Model<NdArray>{
    fn eq(&self, _rhs:&Model<NdArray>)-> bool{true}
}


fn action_mask<B: Backend>(game:&GameState, device: &B::Device) -> Tensor<B,2>{ 
    let mut mask = [0.0;9];
    match game.activeboard{ 
        10 | 9 => {for (index,val) in game.mainboard[9].iter().enumerate(){if *val != 0 {mask[index] = 50.0}}},
        ab => {for (index,val) in game.mainboard[ab].iter().enumerate(){if *val != 0 {mask[index] = 50.0}}}
    }
    Tensor::from_floats([mask], device)
}

#[derive(Module,Debug)]
pub struct Model<B: Backend> {
    lin1: Linear<B>,
    lin2: Linear<B>,
    lin3: Linear<B>,
    lin4: Linear<B>,
    lin5: Linear<B>,
    activation: Relu,
}

impl<B: Backend> Model<B>{
    pub fn new(device: &B::Device) -> Self {
        let lin1 = LinearConfig::new(92,255).init(device);
        let lin2 = LinearConfig::new(255,255).init(device);
        let lin3 = LinearConfig::new(255,100).init(device);
        let lin4 = LinearConfig::new(100,50).init(device);
        let lin5 = LinearConfig::new(50,9).init(device);
        let activation = Relu::new();

        Self {
            lin1,
            lin2,
            lin3,
            lin4,
            lin5,
            activation,
        }
    }

    pub fn forward(&self, input: Tensor<B,2>,mask:&GameState) -> Tensor<B,2>{
        let input = self.lin1.forward(input);
        let input = self.lin2.forward(input);
        let input = self.lin3.forward(input);
        let input = self.lin4.forward(input);
        let input = self.lin5.forward(input);
        let input = self.activation.forward(input);
        let mask = action_mask(mask,&self.devices()[0]);
        input.sub(mask)
    }

    pub fn device(&self) -> B::Device {
        self.devices()[0].clone()
    }

}



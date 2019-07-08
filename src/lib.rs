extern crate nom;
extern crate rand;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

/*
extern crate rusty_machine as rm;
extern crate serde;
extern crate serde_json;
*/
pub mod book;
pub mod color;
pub mod command_parser;
pub mod eval;
pub mod eval_fun;
pub mod global;
pub mod hash;
pub mod make_train_data;
pub mod play;
pub mod print;
pub mod rotate;
pub mod solver;
pub mod think;
pub mod train;

// cargo test -- --nocapture
#[cfg(test)]
mod tests {

    use crate::eval::*;
    use crate::play::*;
    use crate::print::*;
    use crate::rotate::*;
    use crate::solver::*;
    use crate::train::*;
    use rm::learning::nnet::MSECriterion;

    use rm::learning::nnet::NeuralNet;
    use rm::learning::optim::grad_desc::StochasticGD;
    use rm::learning::toolkit::regularization::Regularization;
    use rm::learning::SupModel;
    use rm::linalg::Matrix;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{BufWriter, Write};
    #[test]
    fn it_works() {
        /**/
        /*
        let next: u64 = 4048832882159910912;
        let pre: u64 = 18049789443178496;
        let mut train = Vec::new();
        board_to_bitvec(&mut train, next, pre);

        let def: f64 = 5.0;
        let mobility: f64 = 10.0;
        train.push(def);
        train.push(mobility);

        board_to_bitvec(&mut train, flip_diag_a1h8(&next), flip_diag_a1h8(&pre));
        train.push(def);
        train.push(mobility);

        board_to_bitvec(
            &mut train,
            rotate180(&flip_diag_a1h8(&next)),
            rotate180(&flip_diag_a1h8(&pre)),
        );
        train.push(def);
        train.push(mobility);

        board_to_bitvec(&mut train, rotate180(&next), rotate180(&pre));
        train.push(def);
        train.push(mobility);

        let result_in_next_view: f64 = 4.0;

        let inputs = Matrix::new(4, 130, train);
        let targets = Matrix::new(
            4,
            1,
            vec![
                result_in_next_view,
                result_in_next_view,
                result_in_next_view,
                result_in_next_view,
            ],
        );

        // Set the layer sizes - from input to output
        let layers = &[130, 80, 10, 1]; // 1秒で2回

        let criterion = MSECriterion::new(Regularization::L2(0.3f64));

        // We will just use the default stochastic gradient descent.
        #[derive(Serialize, Deserialize)]
        let mut model = NeuralNet::new(layers, criterion, StochasticGD::default());
        println!("train");
        // Train the model!
        model.train(&inputs, &targets).unwrap();
        println!("train end");

        let next: u64 = 4048832882159910912;
        let pre: u64 = 18049789443178496;
        let mut train = Vec::new();
        board_to_bitvec(&mut train, next, pre);

        let def: f64 = 5.0;
        let mobility: f64 = 10.0;
        train.push(def);
        train.push(mobility);
        let test_inputs = Matrix::new(1, 130, train);

        // And predict new output from the test inputs
        let outputs = model.predict(&test_inputs).unwrap();
        println!("{}", outputs);
        for i in 0..20 {
            model.train(&inputs, &targets).unwrap();
            println!("{}", i);
        }

        let outputs = model.predict(&test_inputs).unwrap();
        println!("{}", outputs);

        let json_str = serde_json::to_string(&model).unwrap();
        let f = File::create("model/test_model.txt").unwrap();
        write!(f, "{}", json_str);


        println!("point: {}", sub_simple_eval(board.black));

        print_board(&board);
        print_unilateral(&solve(
            0b0000000100000001001100010010001100000011000000010000000100000000,
            0b1001111001111110110011101101110011111100000111100000100000000000,
            20,
        ))

         */

        /*
        // let parsed = crate::command_parser::command_parse("OPEN player12345\n");
        // let parsed = crate::command_parser::command_parse("START BLACK Player2 500000\n");
        // let parsed = crate::command_parser::command_parse("END WIN 48 16 DOUBLE_PASS\n");
        let parsed = crate::command_parser::command_parse("BYE Player2 4 4 0 rusThello -4 0 4\n");
        // let parsed = crate::command_parser::command_parse("MOVE PASS\n");
        // let parsed = crate::command_parser::command_parse("MOVE GIVEUP\n");
        // let parsed = crate::command_parser::command_parse("MOVE B3\n");
        // let parsed = crate::command_parser::command_parse("ACK 1000\n");
        match parsed {
            Ok((_, crate::command_parser::Message::Open{name})) => println!("OPEN {}",name),
            Ok((_, crate::command_parser::Message::Start{color,name,time})) => println!("START {}:{}:{}",color, name, time),
            Ok((_, crate::command_parser::Message::End{win_lose,n, m, reason})) => println!("END {}:{}:{}:{}",win_lose,n, m, reason),
            Ok((_, crate::command_parser::Message::Bye{stats})) => println!("Bye {}:{}:{}:{} {}:{}:{}:{}",stats[0].participant, stats[0].score, stats[0].win, stats[0].lose, stats[1].participant, stats[1].score, stats[1].win, stats[1].lose),
            Ok((_, crate::command_parser::Message::Pass)) => println!("Pass"),
            Ok((_, crate::command_parser::Message::Move{x,y})) => println!("Move x:{}, y:{}",x,y),
            Ok((_, crate::command_parser::Message::Ack{time})) => println!("Ack time:{}",time),
            _ => panic!("crash and burn")
        }
        */
    }
}

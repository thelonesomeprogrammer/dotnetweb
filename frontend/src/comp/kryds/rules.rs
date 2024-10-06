use yew::{use_state,Html,html,Callback,function_component,classes};


#[function_component(Rules)]
pub fn main() -> Html {
    let open = use_state(|| false);
    let onclick = {
        let open = open.clone();
        Callback::from( move |_| {
            open.set(!(*open))
        })
    };

    let class1 = if *open{"butline active-1"}else{"butline butline-1"};
    let class2 = if *open{"butline active-2"}else{"butline butline-2"};

    return html!{
        <div class="ultimate-tic-tac-toe-rules-con">
            <button class="ultimate-tic-tac-toe-rules-but" {onclick}>
                <h2>{"Ultimate Tic-Tac-Toe Rules"}</h2>
                <div class = "but-con">
                    <div class = {classes!(class1)}></div>
                    <div class = {classes!(class2)}></div>
                </div>
            </button> 
            <div class="ultimate-tic-tac-toe-rules" style={if *open{""}else{"display:none;"}}>
                <h3>{"1. Player Roles"}</h3>
                <ul>
                    <li>
                        <strong>{"Two Players:"}</strong><br />
                        <p>{"The game is played by two players, Player X and Player O. They alternate placing their marks on the board, attempting to win individual grids and the overall game."}</p>
                    </li>
                </ul>

                <h3>{"2. Objective"}</h3>
                <ul>
                    <li>
                        <strong>{"Win Individual Grids:"}</strong><br />
                        <p>{"Each player aims to win a 3x3 grid by getting three of their marks in a row, column, or diagonal within that grid."}</p>
                    </li>
                    <li>
                        <strong>{"Win the Overall Game:"}</strong><br />
                        <p>{"The ultimate goal is to win three individual grids in a row, column, or diagonal across the larger 3x3 board."}</p>
                    </li>
                </ul>
                
                <h3>{"3. Gameplay Mechanics"}</h3>
                <ul>
                    <li>
                        <strong>{"First Move:"}</strong><br />
                        <p>{"The first player (Player X) select which smaller 3x3 grid the second player (Player O) has to place their first mark"}</p>
                    </li>
                    <li>
                        <strong>{"Subsequent Moves:"}</strong><br />
                        <p>{"The location of a previous player’s mark dictates which smaller grid the opponent must play in next."}</p>
                    </li>
                    <li>
                        <strong>{"Winning a Grid:"}</strong><br />
                        <p>{"A player wins a grid by getting three in a row within that specific 3x3 grid. That grid is then won and marked with X or O."}</p>
                    </li>
                    <li>
                        <strong>{"Grid Already Won:"}</strong><br />
                        <p>{"If the next directed grid has already been won or is a draw, the next player choose what smaller grid they wont to play in"}</p>
                    </li>
                    <li>
                        <strong>{"Win the Game:"}</strong><br />
                        <p>{"A player wins the game by securing three grids in a row, column, or diagonal in the larger 3x3 grid."}</p>
                    </li>
                </ul>
                
                <h3>{"4. Draws"}</h3>
                <ul>
                    <li>
                        <strong>{"Draw in a Grid:"}</strong><br />
                        <p>{"If a grid results in a draw, it counts as neutral but remains in play, until the 3x3 is fully populated"}</p>
                    </li>
                    <li>
                        <strong>{"Overall Draw:"}</strong><br />
                        <p>{"If all grids are filled and neither player has three in a row, the game ends in a draw."}</p>
                    </li>
                </ul>
            </div>
        </div>
    }
}


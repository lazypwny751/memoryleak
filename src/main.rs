use memoryleak::fsm::fsm::GameLoop;

fn main() {
	let state: GameLoop = GameLoop::LoadGame;
	loop {
		println!("{:?}", state);
		match state {
			GameLoop::LoadGame => (),
			GameLoop::SelectSave => (),
			GameLoop::MainMenu => (),
			GameLoop::InGame => (),
			GameLoop::Menu => (),
			GameLoop::GameOver => (),
		}
		break;
	}
}

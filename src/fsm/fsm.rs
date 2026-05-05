// fsm - Fİnite State Machine.
// This file includes:
// - GameLoop: main game state that directs primary events/actions.

// MOTTO: 
//   İmkânın sınırını görmek için, imkânsızı denemek lazım. 
//                                           --Fatih Sultan Mehmet

#[derive(Debug)]
pub enum GameLoop {
	LoadGame,
	SelectSave,
	MainMenu,
	InGame,
	Menu,
	GameOver
}

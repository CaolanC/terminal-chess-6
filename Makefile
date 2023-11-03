build:
	rustc src/main.rs --out-dir bin
	./bin/main

Board:
	rustc src/game/board.rs --out-dir bin
	./bin/board

Piece:
	rustc src/game/piece.rs --out-dir bin
	./bin/piece

Fen:
	rustc src/util/fen_parser.rs --out-dir bin
	./bin/fen_parser

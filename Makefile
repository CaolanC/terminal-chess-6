#build:
#	rustc main.rs
#	./main

Board:
	rustc board.rs --out-dir binaries
	./binaries/board

Piece:
	rustc piece.rs --out-dir binaries
	./binaries/piece

Fen:
	rustc fen_parser.rs --out-dir binaries
	./binaries/fen_parser

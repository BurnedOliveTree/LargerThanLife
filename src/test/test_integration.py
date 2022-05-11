from rust import Engine, Rules


def test_rust_loads_from_python():
    path = "./res/boards/l_test_blinker.csv"
    engine = Engine(Rules.from_str("random chars to make rust read from default"), 600, path)
    assert engine.board() == [[0, 0, 0], [1, 1, 1], [0, 0, 0]]
    engine.update()
    assert engine.board() == [[0, 1, 0], [0, 1, 0], [0, 1, 0]]
    engine.update()
    assert engine.board() == [[0, 0, 0], [1, 1, 1], [0, 0, 0]]
    engine.update()
    assert engine.board() == [[0, 1, 0], [0, 1, 0], [0, 1, 0]]
    engine.update()
    assert engine.board() == [[0, 0, 0], [1, 1, 1], [0, 0, 0]]
    engine.update()
    assert engine.board() == [[0, 1, 0], [0, 1, 0], [0, 1, 0]]
    engine.update()
    assert engine.board() == [[0, 0, 0], [1, 1, 1], [0, 0, 0]]
    engine.update()
    assert engine.board() == [[0, 1, 0], [0, 1, 0], [0, 1, 0]]
    engine.update()
    assert engine.board() == [[0, 0, 0], [1, 1, 1], [0, 0, 0]]
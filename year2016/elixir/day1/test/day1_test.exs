defmodule Day1Test do
  use ExUnit.Case

  test "manhattan distance" do
    assert Day1.manhattan_distance({5, -2}) == 7
    assert Day1.manhattan_distance({-5, 4}) == 9
  end

  test "apply action should rotate and increase position" do
    position = {5, 4}
    direction = {0, -1}
    move = "R4"
    assert Day1.apply_action(move, {position, direction}) == {{1, 4}, {-1, 0}}
  end

  test "apply movement should increase position" do
    position = {5, 4}
    direction = {-1, 0}
    move = "R4"
    assert Day1.apply_movement(move, position, direction) == {1, 4}
  end

  test "given R then rotate right" do
    input = {0, 1}
    assert Day1.rotate("R4", input) == {1, 0}
  end

  test "given L then rotate left" do
    input = {0, 1}
    assert Day1.rotate("L4", input) == {-1, 0}
  end

  test "given F then raise error" do
    input = {0, 1}
    assert_raise CaseClauseError, fn -> Day1.rotate("F4", input) end
  end
end

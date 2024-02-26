defmodule Day8Test do
  use ExUnit.Case

  test "parse_numbers" do
    assert {0, 1} == Grid.parse_numbers("0 by 1", " by ")
    assert {99, 78} == Grid.parse_numbers("99 by 78", " by ")
    assert {8, 5} == Grid.parse_numbers("8x5", "x")
    assert {90, 12} == Grid.parse_numbers("90x12", "x")
  end

  test "turn on lights" do
    # Arrange
    before = %{
      {0, 0} => ".",
      {0, 1} => ".",
      {0, 2} => ".",
      {1, 0} => ".",
      {1, 1} => ".",
      {1, 2} => ".",
      {2, 0} => ".",
      {2, 1} => ".",
      {2, 2} => "."
    }

    expected = %{
      {0, 0} => "#",
      {0, 1} => "#",
      {0, 2} => "#",
      {1, 0} => "#",
      {1, 1} => "#",
      {1, 2} => "#",
      {2, 0} => ".",
      {2, 1} => ".",
      {2, 2} => "."
    }

    # Act
    actual = Grid.turn_on_lights(before, 3, 2)

    ## Assert
    assert expected == actual
  end

  test "rotate column" do
    # Arrange
    input = [{{0, 1}, "#"}, {{1, 1}, "#"}, {{2, 1}, "#"}, {{3, 1}, "."}, {{4, 1}, "."}]
    expected = [{{0, 1}, "#"}, {{1, 1}, "."}, {{2, 1}, "."}, {{3, 1}, "#"}, {{4, 1}, "#"}]

    # Act
    actual = Grid.rotate_column(input, 3)

    # Assert
    assert expected == actual
  end

  test "rotate row" do
    # Arrange
    input = [{{0, 0}, "#"}, {{0, 1}, "#"}, {{0, 2}, "#"}, {{0, 3}, "."}, {{0, 4}, "."}]
    expected = [{{0, 0}, "#"}, {{0, 1}, "."}, {{0, 2}, "."}, {{0, 3}, "#"}, {{0, 4}, "#"}]

    # Act
    actual = Grid.rotate_row(input, 3)

    # Assert
    assert expected == actual
  end

  test "rotate" do
    # Arrange
    input = [{{0, 0}, "#"}, {{1, 0}, "#"}, {{2, 0}, "#"}, {{3, 0}, "."}, {{4, 0}, "."}]
    expected = [{{2, 0}, "#"}, {{3, 0}, "."}, {{4, 0}, "."}, {{0, 0}, "#"}, {{1, 0}, "#"}]

    # Act
    actual = Grid.rotate(input, length(input) - 3)

    # Assert
    assert expected == actual
  end

  test "update grid" do
    # Arrange
    before = %{
      {0, 0} => ".",
      {0, 1} => ".",
      {0, 2} => ".",
      {1, 0} => ".",
      {1, 1} => ".",
      {1, 2} => ".",
      {2, 0} => ".",
      {2, 1} => ".",
      {2, 2} => "."
    }

    update = [{{0, 0}, "#"}, {{1, 0}, "#"}, {{2, 0}, "#"}]

    expected = %{
      {0, 0} => "#",
      {0, 1} => ".",
      {0, 2} => ".",
      {1, 0} => "#",
      {1, 1} => ".",
      {1, 2} => ".",
      {2, 0} => "#",
      {2, 1} => ".",
      {2, 2} => "."
    }

    # Act
    actual = Grid.update(update, before)

    # Assert
    assert expected == actual
  end

  test "get column" do
    map = %{
      {0, 0} => ".",
      {0, 1} => ".",
      {0, 2} => ".",
      {1, 0} => ".",
      {1, 1} => ".",
      {1, 2} => ".",
      {2, 0} => ".",
      {2, 1} => ".",
      {2, 2} => "."
    }

    row = Grid.get_column(map, 2)

    assert [{{0, 2}, "."}, {{1, 2}, "."}, {{2, 2}, "."}] == row
  end

  test "get row" do
    map = %{
      {0, 0} => "#",
      {0, 1} => ".",
      {0, 2} => ".",
      {1, 0} => ".",
      {1, 1} => "#",
      {1, 2} => ".",
      {2, 0} => ".",
      {2, 1} => ".",
      {2, 2} => "#"
    }

    row = Grid.get_row(map, 1)

    assert [{{1, 0}, "."}, {{1, 1}, "#"}, {{1, 2}, "."}] == row
  end
end

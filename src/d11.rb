def iter_neighbors(arr, row, col)
  Enumerator.new do |enum|
    if row > 0
      if col > 0
        enum.yield arr[row - 1][col - 1]
      end
      enum.yield arr[row - 1][col]
      if col < arr[row - 1].size - 1
        enum.yield arr[row - 1][col + 1]
      end
    end

    if row < arr.size - 1
      if col > 0
        enum.yield arr[row + 1][col - 1]
      end
      enum.yield arr[row + 1][col]
      if col < arr[row + 1].size - 1
        enum.yield arr[row + 1][col + 1]
      end
    end

    if col > 0
      enum.yield arr[row][col - 1]
    end
    if col < arr[row].size - 1
      enum.yield arr[row][col + 1]
    end
  end
end

def iter_visible(arr, row, col)
  Enumerator.new do |enum|
    # upper left
    offset = 1
    while row - offset >= 0 && col - offset >= 0
      if arr[row - offset][col - offset] != "."
        enum.yield arr[row - offset][col - offset]
        break
      end
      offset += 1
    end
    # upper
    offset = 1
    while row - offset >= 0
      if arr[row - offset][col] != "."
        enum.yield arr[row - offset][col]
        break
      end
      offset += 1
    end
    # upper right
    offset = 1
    while row - offset >= 0 && col + offset < arr[row].size
      if arr[row - offset][col + offset] != "."
        enum.yield arr[row - offset][col + offset]
        break
      end
      offset += 1
    end
    # left
    offset = 1
    while col - offset >= 0
      if arr[row][col - offset] != "."
        enum.yield arr[row][col - offset]
        break
      end
      offset += 1
    end
    # right
    offset = 1
    while col + offset < arr[row].size
      if arr[row][col + offset] != "."
        enum.yield arr[row][col + offset]
        break
      end
      offset += 1
    end
    # lower left
    offset = 1
    while row + offset < arr.size && col - offset >= 0
      if arr[row + offset][col - offset] != "."
        enum.yield arr[row + offset][col - offset]
        break
      end
      offset += 1
    end
    # lower
    offset = 1
    while row + offset < arr.size
      if arr[row + offset][col] != "."
        enum.yield arr[row + offset][col]
        break
      end
      offset += 1
    end
    # lower right
    offset = 1
    while row + offset < arr.size && col + offset < arr[row].size
      if arr[row + offset][col + offset] != "."
        enum.yield arr[row + offset][col + offset]
        break
      end
      offset += 1
    end
  end
end

def next_state(arr, ri, ci)
  case arr[ri][ci]
    in "."
      return "."
    in "L" if iter_neighbors(arr, ri, ci).none? { |n| n == "#" }
      return "#"
    in "#" if iter_neighbors(arr, ri, ci).count { |n| n == "#" } >= 4
      return "L"
    in x
      return x
  end
end

def next_state_sight_lines(arr, ri, ci)
  case arr[ri][ci]
    in "."
      return "."
    in "L" if iter_visible(arr, ri, ci).none? { |n| n == "#" }
      return "#"
    in "#" if iter_visible(arr, ri, ci).count { |n| n == "#" } >= 5
      return "L"
    in x
      return x
  end
end

def iter_once(arr)
  next_arr = arr.map { |r| r.map { |c| c } }
  arr.each_with_index do |row, ri|
    row.each_index do |ci|
      next_arr[ri][ci] = next_state(arr, ri, ci)
    end
  end
  next_arr
end

def iter_once_sight_lines(arr)
  next_arr = arr.map { |r| r.map { |c| c } }
  arr.each_with_index do |row, ri|
    row.each_index do |ci|
      next_arr[ri][ci] = next_state_sight_lines(arr, ri, ci)
    end
  end
  next_arr
end

def stringify(arr)
  arr.map { |r| r.join("") }.join("\n")
end

input = File.read("./data/d11.txt").lines(chomp: true)
  .map(&:chars)

last = [[]]
curr = input

while stringify(last) != stringify(curr)
  last = curr
  curr = iter_once(curr)
end

puts "Part 1:"
puts curr.map { |r| r.count { |c| c == "#" } }.sum

puts "Part 2:"

last = [[]]
curr = input

while stringify(last) != stringify(curr)
  last = curr
  curr = iter_once_sight_lines(curr)
end
puts curr.map { |r| r.count { |c| c == "#" } }.sum

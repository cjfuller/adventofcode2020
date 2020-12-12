inputs = File.read("./data/d10.txt")
  .lines(chomp: true)
  .map { |line|
    line.to_i
  }
  .sort

inputs = [0] + inputs

puts "Part 1:"
pairs = inputs[...-1].zip(inputs[1...])
puts pairs.count { |i0, i1| i1 - i0 == 1 } *
  (1 + pairs.count { |i0, i1| i1 - i0 == 3 })

puts "Part 2:"

lookup = {inputs[-1] => 1}
inputs[...-1].reverse_each do |i|
  total = lookup.map { |k, v|
    if k - i <= 3
      v
    else
      0
    end
  }.sum

  lookup[i] = total
end
puts lookup[0]

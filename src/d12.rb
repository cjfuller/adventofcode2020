instructions = File.read("./data/d12.txt").lines.map { |l| [l[0], l[1...].to_i] }

state = [0, 0, 0] # angle, N (pos) / S (neg) distance, E (pos) / W (neg) distance

def next_state(curr_state, instruction)
  angle, ns, ew = curr_state
  ins_code, num = instruction
  case ins_code
    in "N"
      [angle, ns + num, ew]
    in "S"
      [angle, ns - num, ew]
    in "E"
      [angle, ns, ew + num]
    in "W"
      [angle, ns, ew - num]
    in "L"
      [(angle + num) % 360, ns, ew]
    in "R"
      [(angle - num) % 360, ns, ew]
    in "F" if angle == 0
      next_state(curr_state, ["E", num])
    in "F" if angle == 90
      next_state(curr_state, ["N", num])
    in "F" if angle == 180
      next_state(curr_state, ["W", num])
    in "F" if angle == 270
      next_state(curr_state, ["S", num])
    in _
      raise "Could not process instruction #{instruction}"
  end
end

instructions.each do |i|
  state = next_state(state, i)
end
puts "Part 1:"
puts state[1].abs + state[2].abs

# ship n/s, ship e/w, way n/s offset, way e/w offset
state = [0, 0, 1, 10]

def next_state_p2(curr_state, instruction)
  ship_ns, ship_ew, way_ns, way_ew = curr_state
  ins_code, num = instruction

  case ins_code
    in "N"
      [ship_ns, ship_ew, way_ns + num, way_ew]
    in "S"
      [ship_ns, ship_ew, way_ns - num, way_ew]
    in "E"
      [ship_ns, ship_ew, way_ns, way_ew + num]
    in "W"
      [ship_ns, ship_ew, way_ns, way_ew - num]
    in "L"
      case num
        in 90
          [ship_ns, ship_ew, way_ew, -way_ns]
        in 180
          [ship_ns, ship_ew, -way_ns, -way_ew]
        in 270
          [ship_ns, ship_ew, -way_ew, way_ns]
        in a
          raise "Don't know how to rotate by angle #{a}"
      end
    in "R"
      next_state_p2(curr_state, ["L", 360 - num])
    in "F"
      [ship_ns + num * way_ns, ship_ew + num * way_ew, way_ns, way_ew]
    in _
      raise "Could not process instruction #{instruction}"
  end
end

instructions.each do |i|
  state = next_state_p2(state, i)
end
puts "Part 2:"
puts state[0].abs + state[1].abs

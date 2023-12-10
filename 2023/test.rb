def first_digit(str)
  str.match(/\d/).to_s
end

def last_digit(str)
  str.reverse.match(/\d/).to_s
end

content = File.read(ARGV[0])

sum = 0
for line in content.split("\n")
  num = "#{first_digit(line)}#{last_digit(line)}"
  sum = sum + num.to_i
end

puts sum

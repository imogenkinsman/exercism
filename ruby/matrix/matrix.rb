=begin
Write your code for the 'Matrix' exercise in this file. Make the tests in
`matrix_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/matrix` directory.
=end

class Matrix
  attr_reader :rows, :columns

  def initialize(string)
    @rows = []

    string.each_line do |line|
      @rows.push(line.split(" ").map(&:to_i))
    end

    @columns = if @rows.empty?
      []
    else
      Array.new(@rows.first.count) { Array.new }
    end

    @rows.each do |row|
      row.each_with_index do |x, i|
        @columns[i].push(x)
      end
    end
  end
end
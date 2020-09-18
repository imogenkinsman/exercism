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

    head, *tail = @rows
    @columns = head.zip(*tail)
  end
end
=begin
Write your code for the 'Series' exercise in this file. Make the tests in
`series_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/series` directory.
=end

class Series
  def initialize(series_string)
    @series = series_string.split('')
  end

  def slices(length)
    raise ArgumentError if length > @series.length
    
    @series.each_cons(length).map{|con| con.join}
  end
end
# Declare package 'Leap'
package Leap;
use strict;
use warnings;
use Exporter 'import';
our @EXPORT_OK = qw(is_leap);

sub is_leap {
  my ($year) = @_;
  if ($year % 4 == 0) { #on every year that is evenly divisible by 4
  	return 1 unless ($year % 100 == 0); #except every year that is evenly divisible by 100
  	return 1 if ($year % 400 == 0);	#unless the year is also evenly divisible by 400
  }
  return 0;
}

1;

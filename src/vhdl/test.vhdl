library ieee;
use ieee.std_logic_1164.all;

entity testbench is
end entity testbench;

architecture testbench of testbench is
    signal a : std_logic;
    signal b : std_logic;
    signal c : std_logic;
begin
    a <= '1';
    b <= '0';
    c <= a and b;
end architecture testbench;
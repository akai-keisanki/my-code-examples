-module(players).
-export([join/2, get/1, bonus/2, steal/3, leave/1]).

-record(player_data, {id, name, points = 100}).

loop(PlayerData) ->
  #player_data{id = Id, name = Name, points = Points} = PlayerData,
  receive
  {get, Caller} ->
    Caller ! PlayerData,
    loop(PlayerData);
  {bonus, Additional} ->
    loop(PlayerData#player_data{points = Points + Additional});
  {take, Caller, Amount} ->
    case Points > Amount
    of
    true ->
      Caller ! taken,
      loop(PlayerData#player_data{points = Points - Amount});
    false ->
      Caller ! not_taken,
      loop(PlayerData)
    end;
  {steal, Player2, Amount} ->
    Player2 ! {take, self(), Amount},
    receive
    taken ->
      loop(PlayerData#player_data{points = Points + Amount});
    _ ->
      case Points > Amount
      of
        true ->
          loop(PlayerData#player_data{points = Points - Amount});
        false ->
          loop(PlayerData)
      end
    end;
  leave ->
    ok
  end.

join(Id, Name) ->
  spawn(fun() -> loop(#player_data{id = Id, name = Name}) end).

get(Player) ->
  Player ! {get, self()},
  receive
  PlayerData -> PlayerData
  end.

bonus(Player, Additional) ->
  Player ! {bonus, Additional}.

steal(Player, Player2, Amount) ->
  Player ! {steal, Player2, Amount}.

leave(Player) ->
  Player ! leave.

defmodule Player.Data
do
  defstruct [:id, :name, points: 100]
end

defmodule Player
do
  def join(id, name)
  do
    spawn(fn -> loop(%Player.Data{id: id, name: name}) end)
  end

  defp loop(player_data)
  do
    %{id: id, name: name, points: points} = player_data

    receive do
    {:get, caller} -> 
      send(caller, player_data)
      loop(player_data)
    {:bonus, additional} ->
      loop(%{player_data | points: points + additional})
    {:take, caller, amount} ->
      if points > amount
      do
        send(caller, :taken)
        loop(%{player_data | points: points - amount})
      else
        send(caller, :not_taken)
        loop(player_data)
      end
    {:steal, player2, amount} ->
      send(player2, {:take, self(), amount})
      receive do
      :taken ->
        loop(%{player_data | points: points + amount})
      _ ->
        if points > amount
        do
          loop(%{player_data | points: points - amount})
        else
          loop(player_data)
        end
      end
    :leave ->
      :ok
    _ ->
      loop(player_data)
    end
  end

  def get(player)
  do
    send(player, {:get, self()})
    
    receive do
    player_data -> player_data
    end
  end

  def bonus(player, additional)
  do
    send(player, {:bonus, additional})
  end

  def steal(player, player2, amount)
  do
    send(player, {:steal, player2, amount})
  end

  def leave(player)
  do
    send(player, :leave)
  end
end

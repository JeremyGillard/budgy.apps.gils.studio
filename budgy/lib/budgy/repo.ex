defmodule Budgy.Repo do
  use Ecto.Repo,
    otp_app: :budgy,
    adapter: Ecto.Adapters.Postgres
end

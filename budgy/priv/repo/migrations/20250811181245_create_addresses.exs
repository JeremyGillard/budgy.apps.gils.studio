defmodule Budgy.Repo.Migrations.CreateAddresses do
  use Ecto.Migration

  def change do
    create table(:addresses, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :street, :string
      add :number, :string
      add :postal_code, :string
      add :city, :string

      timestamps(type: :utc_datetime)
    end

    create unique_index(:addresses, [:street, :number, :postal_code, :city])
  end
end

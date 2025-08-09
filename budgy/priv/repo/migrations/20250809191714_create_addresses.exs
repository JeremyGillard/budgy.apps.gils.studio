defmodule Budgy.Repo.Migrations.CreateAddresses do
  use Ecto.Migration

  def change do
    create table(:addresses, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :street, :string, null: false
      add :number, :string, null: false
      add :postal_code, :string, null: false
      add :city, :string, null: false

      timestamps(type: :utc_datetime)
    end

    create unique_index(:addresses, [:street, :number, :postal_code, :city])
  end
end

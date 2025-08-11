defmodule Budgy.Repo.Migrations.CreateBanks do
  use Ecto.Migration

  def change do
    create table(:banks, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :name, :string
      add :bic, :string, null: false
      add :country_code, :string

      timestamps(type: :utc_datetime)
    end

    create unique_index(:banks, :bic)
    create unique_index(:banks, [:name, :bic])
  end
end

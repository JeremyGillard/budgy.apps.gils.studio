defmodule Budgy.Repo.Migrations.CreateCounterparties do
  use Ecto.Migration

  def change do
    create table(:counterparties, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :name, :string
      add :address_id, references(:addresses, on_delete: :nothing, type: :binary_id)

      timestamps(type: :utc_datetime)
    end

    create index(:counterparties, [:address_id])
    create unique_index(:counterparties, :name)
  end
end

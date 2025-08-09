defmodule Budgy.Repo.Migrations.CreateCounterparties do
  use Ecto.Migration

  def change do
    create table(:counterparties, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :name, :string
      add :account, :string, null: false
      add :bank_id, references(:banks, on_delete: :nothing, type: :binary_id)
      add :address_id, references(:addresses, on_delete: :nothing, type: :binary_id)

      timestamps(type: :utc_datetime)
    end

    create index(:counterparties, [:bank_id])
    create index(:counterparties, [:address_id])
  end
end

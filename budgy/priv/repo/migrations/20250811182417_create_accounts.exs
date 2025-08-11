defmodule Budgy.Repo.Migrations.CreateAccounts do
  use Ecto.Migration

  def change do
    create table(:accounts, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :number, :string, null: false
      add :bank_id, references(:banks, on_delete: :nothing, type: :binary_id)
      add :counterparty_id, references(:counterparties, on_delete: :nothing, type: :binary_id)

      timestamps(type: :utc_datetime)
    end

    create unique_index(:accounts, [:number])
    create index(:accounts, [:bank_id])
    create index(:accounts, [:counterparty_id])
  end
end

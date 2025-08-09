defmodule Budgy.BankingTest do
  use Budgy.DataCase

  alias Budgy.Banking

  describe "addresses" do
    alias Budgy.Banking.Address

    import Budgy.BankingFixtures

    @invalid_attrs %{number: nil, street: nil, postal_code: nil, city: nil}

    test "list_addresses/0 returns all addresses" do
      address = address_fixture()
      assert Banking.list_addresses() == [address]
    end

    test "get_address!/1 returns the address with given id" do
      address = address_fixture()
      assert Banking.get_address!(address.id) == address
    end

    test "create_address/1 with valid data creates a address" do
      valid_attrs = %{number: "some number", street: "some street", postal_code: "some postal_code", city: "some city"}

      assert {:ok, %Address{} = address} = Banking.create_address(valid_attrs)
      assert address.number == "some number"
      assert address.street == "some street"
      assert address.postal_code == "some postal_code"
      assert address.city == "some city"
    end

    test "create_address/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Banking.create_address(@invalid_attrs)
    end

    test "update_address/2 with valid data updates the address" do
      address = address_fixture()
      update_attrs = %{number: "some updated number", street: "some updated street", postal_code: "some updated postal_code", city: "some updated city"}

      assert {:ok, %Address{} = address} = Banking.update_address(address, update_attrs)
      assert address.number == "some updated number"
      assert address.street == "some updated street"
      assert address.postal_code == "some updated postal_code"
      assert address.city == "some updated city"
    end

    test "update_address/2 with invalid data returns error changeset" do
      address = address_fixture()
      assert {:error, %Ecto.Changeset{}} = Banking.update_address(address, @invalid_attrs)
      assert address == Banking.get_address!(address.id)
    end

    test "delete_address/1 deletes the address" do
      address = address_fixture()
      assert {:ok, %Address{}} = Banking.delete_address(address)
      assert_raise Ecto.NoResultsError, fn -> Banking.get_address!(address.id) end
    end

    test "change_address/1 returns a address changeset" do
      address = address_fixture()
      assert %Ecto.Changeset{} = Banking.change_address(address)
    end
  end

  describe "banks" do
    alias Budgy.Banking.Bank

    import Budgy.BankingFixtures

    @invalid_attrs %{name: nil, bic: nil, country_code: nil}

    test "list_banks/0 returns all banks" do
      bank = bank_fixture()
      assert Banking.list_banks() == [bank]
    end

    test "get_bank!/1 returns the bank with given id" do
      bank = bank_fixture()
      assert Banking.get_bank!(bank.id) == bank
    end

    test "create_bank/1 with valid data creates a bank" do
      valid_attrs = %{name: "some name", bic: "some bic", country_code: "some country_code"}

      assert {:ok, %Bank{} = bank} = Banking.create_bank(valid_attrs)
      assert bank.name == "some name"
      assert bank.bic == "some bic"
      assert bank.country_code == "some country_code"
    end

    test "create_bank/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Banking.create_bank(@invalid_attrs)
    end

    test "update_bank/2 with valid data updates the bank" do
      bank = bank_fixture()
      update_attrs = %{name: "some updated name", bic: "some updated bic", country_code: "some updated country_code"}

      assert {:ok, %Bank{} = bank} = Banking.update_bank(bank, update_attrs)
      assert bank.name == "some updated name"
      assert bank.bic == "some updated bic"
      assert bank.country_code == "some updated country_code"
    end

    test "update_bank/2 with invalid data returns error changeset" do
      bank = bank_fixture()
      assert {:error, %Ecto.Changeset{}} = Banking.update_bank(bank, @invalid_attrs)
      assert bank == Banking.get_bank!(bank.id)
    end

    test "delete_bank/1 deletes the bank" do
      bank = bank_fixture()
      assert {:ok, %Bank{}} = Banking.delete_bank(bank)
      assert_raise Ecto.NoResultsError, fn -> Banking.get_bank!(bank.id) end
    end

    test "change_bank/1 returns a bank changeset" do
      bank = bank_fixture()
      assert %Ecto.Changeset{} = Banking.change_bank(bank)
    end
  end

  describe "counterparties" do
    alias Budgy.Banking.Counterparty

    import Budgy.BankingFixtures

    @invalid_attrs %{name: nil, account: nil}

    test "list_counterparties/0 returns all counterparties" do
      counterparty = counterparty_fixture()
      assert Banking.list_counterparties() == [counterparty]
    end

    test "get_counterparty!/1 returns the counterparty with given id" do
      counterparty = counterparty_fixture()
      assert Banking.get_counterparty!(counterparty.id) == counterparty
    end

    test "create_counterparty/1 with valid data creates a counterparty" do
      valid_attrs = %{name: "some name", account: "some account"}

      assert {:ok, %Counterparty{} = counterparty} = Banking.create_counterparty(valid_attrs)
      assert counterparty.name == "some name"
      assert counterparty.account == "some account"
    end

    test "create_counterparty/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Banking.create_counterparty(@invalid_attrs)
    end

    test "update_counterparty/2 with valid data updates the counterparty" do
      counterparty = counterparty_fixture()
      update_attrs = %{name: "some updated name", account: "some updated account"}

      assert {:ok, %Counterparty{} = counterparty} = Banking.update_counterparty(counterparty, update_attrs)
      assert counterparty.name == "some updated name"
      assert counterparty.account == "some updated account"
    end

    test "update_counterparty/2 with invalid data returns error changeset" do
      counterparty = counterparty_fixture()
      assert {:error, %Ecto.Changeset{}} = Banking.update_counterparty(counterparty, @invalid_attrs)
      assert counterparty == Banking.get_counterparty!(counterparty.id)
    end

    test "delete_counterparty/1 deletes the counterparty" do
      counterparty = counterparty_fixture()
      assert {:ok, %Counterparty{}} = Banking.delete_counterparty(counterparty)
      assert_raise Ecto.NoResultsError, fn -> Banking.get_counterparty!(counterparty.id) end
    end

    test "change_counterparty/1 returns a counterparty changeset" do
      counterparty = counterparty_fixture()
      assert %Ecto.Changeset{} = Banking.change_counterparty(counterparty)
    end
  end
end

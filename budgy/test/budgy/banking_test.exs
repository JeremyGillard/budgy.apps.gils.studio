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
end

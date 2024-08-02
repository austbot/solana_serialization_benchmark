/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type ReadCollectionBincodeInstructionAccounts = {
  /** The address of the new account */
  address: PublicKey | Pda;
};

// Data.
export type ReadCollectionBincodeInstructionData = { discriminator: number };

export type ReadCollectionBincodeInstructionDataArgs = {};

export function getReadCollectionBincodeInstructionDataSerializer(): Serializer<
  ReadCollectionBincodeInstructionDataArgs,
  ReadCollectionBincodeInstructionData
> {
  return mapSerializer<
    ReadCollectionBincodeInstructionDataArgs,
    any,
    ReadCollectionBincodeInstructionData
  >(
    struct<ReadCollectionBincodeInstructionData>([['discriminator', u8()]], {
      description: 'ReadCollectionBincodeInstructionData',
    }),
    (value) => ({ ...value, discriminator: 16 })
  ) as Serializer<
    ReadCollectionBincodeInstructionDataArgs,
    ReadCollectionBincodeInstructionData
  >;
}

// Instruction.
export function readCollectionBincode(
  context: Pick<Context, 'programs'>,
  input: ReadCollectionBincodeInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'solanaSerializationBenchmark',
    'BENCHVr3SC7dVDMtKVpwctjFNPBMrqvXn9JVACJg3KEb'
  );

  // Accounts.
  const resolvedAccounts = {
    address: {
      index: 0,
      isWritable: false as boolean,
      value: input.address ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getReadCollectionBincodeInstructionDataSerializer().serialize(
    {}
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
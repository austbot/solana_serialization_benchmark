/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum ExampleEnum {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

export type ExampleEnumArgs = ExampleEnum;

export function getExampleEnumSerializer(): Serializer<
  ExampleEnumArgs,
  ExampleEnum
> {
  return scalarEnum<ExampleEnum>(ExampleEnum, {
    description: 'ExampleEnum',
  }) as Serializer<ExampleEnumArgs, ExampleEnum>;
}

// FIXME: re-enable tests when new lib-openapi.json merges
// same story as kotlin, before the new lib-openapi.json is merged this test wont compile
package com.svix.test;

import static org.junit.Assert.*;

import com.fasterxml.jackson.core.JsonProcessingException;
//  import com.svix.models.CronConfig;
import com.svix.models.DashboardAccessOut;
//  import com.svix.models.IngestSourceIn;

import org.junit.Test;

public class ModelTests {
    @Test
    public void toJsonWorks() throws JsonProcessingException {
        DashboardAccessOut model = new DashboardAccessOut().token("asd");
        assertEquals("{\"token\":\"asd\"}", model.toJson());
    }

    @Test
    public void fromJsonWorks() throws JsonProcessingException {
        DashboardAccessOut expectedModel = new DashboardAccessOut().token("asd");
        DashboardAccessOut model = DashboardAccessOut.fromJson("{\"token\":\"asd\"}");
        assertEquals(expectedModel, model);
    }

    //  @Test
    //  public void structEnumWithNoExtraFields() throws JsonProcessingException {
    //      String jsonString =
    //              "{\"name\":\"mendy\",\"uid\":\"very"
    //                      + " unique\",\"type\":\"generic-webhook\",\"config\":{}}";
    //      IngestSourceIn sourceIn =
    //              new IngestSourceIn(
    //                      "mendy",
    //                      "very unique",
    //                      new IngestSourceIn.IngestSourceInType.GenericWebhook());

    //      assertEquals(jsonString, sourceIn.toJson());
    //      assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
    //  }

    //  @Test
    //  public void structEnumWithFields() throws JsonProcessingException {
    //      String jsonString =
    //              "{\"name\":\"name\",\"uid\":\"uuiidd\",\"type\":\"cron\",\"config\":{\"contentType\":\"asd\",\"payload\":\"cool\",\"schedule\":\"*"
    //                  + " * * * *\"}}";
    //      IngestSourceIn sourceIn =
    //              new IngestSourceIn(
    //                      "name",
    //                      "uuiidd",
    //                      new IngestSourceIn.IngestSourceInType.Cron(
    //                              new CronConfig()
    //                                      .contentType("asd")
    //                                      .payload("cool")
    //                                      .schedule("* * * * *")));
    //      assertEquals(jsonString, sourceIn.toJson());
    //      assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
    //  }
}
